use crate::schedule::{EndPoint, Interval, Schedule};
use fraction::{ConstZero, Fraction};
use std::iter;

#[derive(Clone, Copy, PartialEq, Eq)]
/// corresponds to C integer encoding in nuXmv translation
pub enum IntegerType {
    XmvInt,
    BitVec,
}

pub trait Xmv {
    fn as_xmv(&self) -> String;
}

enum XmvBinOp {
    Eq,
    Le,
    #[allow(dead_code)]
    Ge,
    Leq,
    #[allow(dead_code)]
    Geq,
    And,
    Implies,
}

impl Xmv for XmvBinOp {
    fn as_xmv(&self) -> String {
        String::from(match self {
            Self::Eq => "=",
            Self::Le => "<",
            Self::Ge => ">",
            Self::Leq => "<=",
            Self::Geq => ">=",
            Self::And => "&",
            Self::Implies => "->",
        })
    }
}

enum XmvExp {
    BinOp(XmvBinOp, Box<XmvExp>, Box<XmvExp>),
    True,
    Int(usize),
    BitInt(usize),
    Real(Fraction),
    ScheduleClock,
    ScheduleState,
    Case(Vec<(XmvExp, XmvExp)>),
    Next(Box<XmvExp>),
}

impl Xmv for XmvExp {
    fn as_xmv(&self) -> String {
        match self {
            Self::BinOp(op, lhs, rhs) => {
                format!("({} {} {})", lhs.as_xmv(), op.as_xmv(), rhs.as_xmv())
            }
            Self::Int(i) => i.to_string(),
            Self::True => String::from("TRUE"),

            // I hope we do not have any issues with invalid fractions...
            Self::Real(f) => format!("f'{}/{}", f.numer().unwrap(), f.denom().unwrap()),

            // reserved keywords by chronos
            Self::ScheduleClock => String::from("schedule_clock"),
            Self::ScheduleState => String::from("\"scheduler_state\""),

            Self::Case(cases) => {
                let cases_strings: Vec<String> = cases
                    .iter()
                    .map(|(cond, exp)| format!("  {} : {};", cond.as_xmv(), exp.as_xmv()))
                    .collect();
                format!("case\n{}\nesac", cases_strings.join("\n"))
            }
            Self::BitInt(i) => {
                format!("0ud32_{i}")
            }
            Self::Next(exp) => format!("next({})", exp.as_xmv()),
        }
    }
}

pub struct Init(XmvExp);

impl Xmv for Init {
    fn as_xmv(&self) -> String {
        let Self(exp) = self;
        format!("INIT {}", exp.as_xmv())
    }
}

impl Init {
    fn zero_clock() -> Self {
        Self(XmvExp::BinOp(
            XmvBinOp::Eq,
            Box::new(XmvExp::ScheduleClock),
            Box::new(XmvExp::Int(0)),
        ))
    }

    fn zero_scheduler(integer_type: IntegerType) -> Self {
        let zero = match integer_type {
            IntegerType::XmvInt => XmvExp::Int(0),
            IntegerType::BitVec => XmvExp::BitInt(0),
        };

        Self(XmvExp::BinOp(
            XmvBinOp::Eq,
            Box::new(XmvExp::ScheduleState),
            Box::new(zero),
        ))
    }
}

pub struct Justice(XmvExp);

impl Xmv for Justice {
    fn as_xmv(&self) -> String {
        let Self(exp) = self;
        format!("JUSTICE {}", exp.as_xmv())
    }
}

impl Justice {
    fn from_schedule(schedule: &Schedule, integer_type: IntegerType) -> Vec<Self> {
        // last index of ScheduleState
        let last_job = schedule.get_number_of_jobs() - 1;

        // expressions for first and last index of ScheduleState
        let (first_job, last_job) = match integer_type {
            IntegerType::XmvInt => (XmvExp::Int(0), XmvExp::Int(last_job)),
            IntegerType::BitVec => (XmvExp::BitInt(0), XmvExp::BitInt(last_job)),
        };

        // justice constraints for scheduler reaching first and last job
        let first_job_justice = Justice(XmvExp::BinOp(
            XmvBinOp::Eq,
            Box::new(XmvExp::ScheduleState),
            Box::new(first_job),
        ));
        let last_job_justice = Justice(XmvExp::BinOp(
            XmvBinOp::Eq,
            Box::new(XmvExp::ScheduleState),
            Box::new(last_job),
        ));

        vec![first_job_justice, last_job_justice]
    }
}

pub struct Invar(XmvExp);

impl Xmv for Invar {
    fn as_xmv(&self) -> String {
        let Self(exp) = self;
        format!("INVAR {}", exp.as_xmv())
    }
}

impl Invar {
    fn from_hyperperiod(hyper_period: Fraction) -> Self {
        Self(XmvExp::BinOp(
            XmvBinOp::Implies,
            Box::new(XmvExp::True),
            Box::new(XmvExp::BinOp(
                XmvBinOp::And,
                Box::new(XmvExp::BinOp(
                    XmvBinOp::Leq,
                    Box::new(XmvExp::Real(Fraction::ZERO)),
                    Box::new(XmvExp::ScheduleClock),
                )),
                Box::new(XmvExp::BinOp(
                    XmvBinOp::Leq,
                    Box::new(XmvExp::ScheduleClock),
                    Box::new(XmvExp::Real(hyper_period)),
                )),
            )),
        ))
    }

    fn from_interval(job_id: usize, interval: Interval, integer_type: IntegerType) -> Self {
        let Interval(left, right) = interval;

        let job_id_exp = match integer_type {
            IntegerType::BitVec => XmvExp::BitInt(job_id),
            IntegerType::XmvInt => XmvExp::Int(job_id),
        };

        // scheduler_state = state
        let state_condition = XmvExp::BinOp(
            XmvBinOp::Eq,
            Box::new(XmvExp::ScheduleState),
            Box::new(job_id_exp),
        );

        // x <= scheduler_clock
        let left_constraint = match left {
            EndPoint::Open(limit) => XmvExp::BinOp(
                XmvBinOp::Le,
                Box::new(XmvExp::Real(limit)),
                Box::new(XmvExp::ScheduleClock),
            ),
            EndPoint::Closed(limit) => XmvExp::BinOp(
                XmvBinOp::Leq,
                Box::new(XmvExp::Real(limit)),
                Box::new(XmvExp::ScheduleClock),
            ),
        };

        // scheduler_clock <= x
        let right_constraint = match right {
            EndPoint::Open(limit) => XmvExp::BinOp(
                XmvBinOp::Le,
                Box::new(XmvExp::ScheduleClock),
                Box::new(XmvExp::Real(limit)),
            ),
            EndPoint::Closed(limit) => XmvExp::BinOp(
                XmvBinOp::Leq,
                Box::new(XmvExp::ScheduleClock),
                Box::new(XmvExp::Real(limit)),
            ),
        };

        // scheduler_state = state -> x <= scheduler_clock & scheduler_clock <= x
        Self(XmvExp::BinOp(
            XmvBinOp::Implies,
            Box::new(state_condition),
            Box::new(XmvExp::BinOp(
                XmvBinOp::And,
                Box::new(left_constraint),
                Box::new(right_constraint),
            )),
        ))
    }
}

pub struct Urgent(XmvExp);

impl Xmv for Urgent {
    fn as_xmv(&self) -> String {
        let Self(exp) = self;
        format!("URGENT {}", exp.as_xmv())
    }
}

impl Urgent {
    /// jobs with interval [a,a] may be constrained with URGENT
    fn from_interval(job_id: usize, interval: Interval, integer_type: IntegerType) -> Option<Self> {
        let job_id_exp = match integer_type {
            IntegerType::BitVec => XmvExp::BitInt(job_id),
            IntegerType::XmvInt => XmvExp::Int(job_id),
        };
        // scheduler_state = state
        let state_condition = XmvExp::BinOp(
            XmvBinOp::Eq,
            Box::new(XmvExp::ScheduleState),
            Box::new(job_id_exp),
        );
        // applicability check
        match interval {
            Interval(EndPoint::Closed(x), EndPoint::Closed(y)) if x == y => {
                Some(Self(state_condition))
            }
            _ => None,
        }
    }
}

pub struct Trans(XmvExp);

impl Xmv for Trans {
    fn as_xmv(&self) -> String {
        let Self(exp) = self;
        format!("TRANS {}", exp.as_xmv())
    }
}

impl Trans {
    /// Generate discrete transition constraints for clock
    fn from_hyperperiod(hyperperiod: Fraction) -> Self {
        Self(XmvExp::Case(vec![
            // c >= period : next(c) = 0;
            (
                XmvExp::BinOp(
                    XmvBinOp::Eq,
                    Box::new(XmvExp::ScheduleClock),
                    Box::new(XmvExp::Real(hyperperiod)),
                ),
                XmvExp::BinOp(
                    XmvBinOp::Eq,
                    Box::new(XmvExp::Next(Box::new(XmvExp::ScheduleClock))),
                    Box::new(XmvExp::Int(0)),
                ),
            ),
            // TRUE : next(c) = c;
            (
                XmvExp::True,
                XmvExp::BinOp(
                    XmvBinOp::Eq,
                    Box::new(XmvExp::Next(Box::new(XmvExp::ScheduleClock))),
                    Box::new(XmvExp::ScheduleClock),
                ),
            ),
        ]))
    }
}

pub struct TimingConstraints {
    invar: Vec<Invar>,
    urgent: Vec<Urgent>,
    trans: Trans,
    init: Vec<Init>,
    justice: Vec<Justice>,
}

impl TimingConstraints {
    /// integer_Type: how integers are encoded by Kratos
    pub fn new(schedule: &Schedule, integer_type: IntegerType) -> Self {
        let hyperperiod = schedule.get_hyper_period();
        let jobs = schedule.get_jobs_with_ids();

        // clock bounds
        let period_invar = Invar::from_hyperperiod(hyperperiod);
        // job interval constraints
        let job_invars = jobs
            .iter()
            .map(|(id, interval)| Invar::from_interval(*id, *interval, integer_type));
        let invar = iter::once(period_invar).chain(job_invars).collect();

        // applicable urgent constraints
        let urgent = jobs
            .iter()
            .filter_map(|(id, interval)| Urgent::from_interval(*id, *interval, integer_type))
            .collect();

        // clock discrete behavior
        let trans = Trans::from_hyperperiod(hyperperiod);

        // clock and scheduler initial value
        // we constrain scheduler also, to minimize init-behavior
        // note that Kratos initializes variables as a "step"
        let init = vec![Init::zero_clock(), Init::zero_scheduler(integer_type)];

        // scheduler fairness (perhaps redundant)
        let justice = Justice::from_schedule(schedule, integer_type);

        Self {
            invar,
            urgent,
            trans,
            init,
            justice,
        }
    }
}

impl<T: Xmv> Xmv for &[T] {
    fn as_xmv(&self) -> String {
        self.iter()
            .map(|i| i.as_xmv())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl Xmv for TimingConstraints {
    fn as_xmv(&self) -> String {
        format!(
            "-- At the top of the xmv file add:\n@TIME_DOMAIN continuous\n-- to the global module add:\nVAR schedule_clock: clock;\n{}\n{}\n{}\n{}\n{}",
            self.init.as_slice().as_xmv(),
            self.trans.as_xmv(),
            self.invar.as_slice().as_xmv(),
            self.urgent.as_slice().as_xmv(),
            self.justice.as_slice().as_xmv(),
        )
    }
}
