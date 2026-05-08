use fraction::{ConstZero, Fraction, Signed};
use lalrpop_util::{ParseError, lalrpop_mod};
use schedule_grammar::Token;
use std::collections::HashSet;
use std::mem;

lalrpop_mod!(
    #[allow(clippy::ptr_arg)]
    #[rustfmt::skip]
    schedule_grammar
);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EndPoint {
    Open(Fraction),
    Closed(Fraction),
}

impl EndPoint {
    fn get_limit(&self) -> Fraction {
        match self {
            Self::Open(l) => *l,
            Self::Closed(l) => *l,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Interval(pub EndPoint, pub EndPoint);

impl Interval {
    fn validate(&self) -> Result<(), ()> {
        let Self(left, right) = self;
        if left.get_limit().is_negative() || right.get_limit().is_negative() {
            return Err(());
        }

        match (left, right) {
            (EndPoint::Closed(l), EndPoint::Open(r))
            | (EndPoint::Open(l), EndPoint::Closed(r))
            | (EndPoint::Open(l), EndPoint::Open(r)) => {
                if l >= r {
                    Err(())
                } else {
                    Ok(())
                }
            }
            (EndPoint::Closed(l), EndPoint::Closed(r)) => {
                if l > r {
                    Err(())
                } else {
                    Ok(())
                }
            }
        }
    }

    /// self shadows other if all time points in other are contained by self
    fn shadows(&self, other: &Self) -> bool {
        let contained_left = match (self, other) {
            (Self(self_l, _), Self(other_l, _)) if self_l == other_l => true,
            (Self(EndPoint::Closed(self_l), _), Self(other_l, _)) => *self_l <= other_l.get_limit(),
            (Self(EndPoint::Open(self_l), _), Self(other_l, _)) => *self_l < other_l.get_limit(),
        };
        let contained_right = match (self, other) {
            (Self(_, self_r), Self(_, other_r)) if self_r == other_r => true,
            (Self(_, EndPoint::Closed(self_r)), Self(_, other_r)) => *self_r >= other_r.get_limit(),
            (Self(_, EndPoint::Open(self_r)), Self(_, other_r)) => *self_r > other_r.get_limit(),
        };

        contained_left && contained_right
    }

    /// intervals that share at least one time point overlap
    fn overlaps(&self, other: &Self) -> bool {
        match (self, other) {
            (Self(EndPoint::Closed(self_l), _), Self(_, EndPoint::Closed(other_r)))
                if self_l == other_r =>
            {
                true
            }
            (Self(_, EndPoint::Closed(self_r)), Self(EndPoint::Closed(other_l), _))
                if self_r == other_l =>
            {
                true
            }
            (Self(self_l, self_r), Self(other_l, other_r)) => {
                let l_max = self_l.get_limit().max(other_l.get_limit());
                let r_min = self_r.get_limit().min(other_r.get_limit());

                l_max < r_min
            }
        }
    }

    /// self is left of other if all time points belonging to self
    /// are less than all time points of other
    fn left_of(&self, other: &Self) -> bool {
        let Self(self_l, _) = self;
        let Self(_, other_r) = other;

        !self.overlaps(other) && self_l.get_limit() < other_r.get_limit()
    }

    /// self is right of other if all time points belonging to self
    /// are greater than all time points of other
    fn right_of(&self, other: &Self) -> bool {
        other.left_of(self)
    }

    /// creates interval self and other, as well as all points in between
    /// self must be before other, and intervals should not overlap, otherwise this is unnecessary
    /// TODO improve
    fn bridge(&self, other: &Self) -> Self {
        let Self(self_l, _) = self;
        let Self(_, other_r) = other;
        Self(*self_l, *other_r)
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub struct Task<'input>(&'input str);

impl Task<'_> {
    pub fn get_name(&self) -> &str {
        let Self(name) = self;
        name
    }
}

pub struct Job<'input>(Option<Task<'input>>, Interval);

impl<'input> Job<'input> {
    fn get_constraint(&self) -> Interval {
        let Self(_, interval) = self;
        *interval
    }

    fn get_task(&self) -> &Option<Task<'input>> {
        let Self(task, _) = self;
        task
    }
}

pub struct Schedule<'input> {
    tasks: HashSet<Task<'input>>,
    jobs: Vec<Job<'input>>, // ordered list of jobs
    hyper_period: Fraction,
}

#[derive(Clone, Copy, Debug)]
pub enum InvalidScheduleError {
    JobWithoutTask,
    InvertedJobOrder,
    InvalidInterval(#[allow(dead_code)] Interval),
    IntervalExceedsHyperPeriod,
    ShadowedSuccessor,
}

impl<'input> Schedule<'input> {
    fn new(tasks: HashSet<Task<'input>>, jobs: Vec<Job<'input>>, hyper_period: Fraction) -> Self {
        Self {
            tasks,
            jobs,
            hyper_period,
        }
    }

    pub fn get_hyper_period(&self) -> Fraction {
        self.hyper_period
    }

    pub fn get_tasks(&self) -> &HashSet<Task<'input>> {
        &self.tasks
    }

    /// get all jobs, with their corresponding id
    pub fn get_jobs_with_ids(&self) -> Vec<(usize, Interval)> {
        self.jobs
            .iter()
            .enumerate()
            .map(|(id, job)| (id, job.get_constraint()))
            .collect()
    }

    pub fn get_number_of_jobs(&self) -> usize {
        self.jobs.len()
    }

    /// get job ids corresponding to a task
    pub fn task_to_job_ids(&self, task: Task) -> Vec<usize> {
        self.jobs
            .iter()
            .enumerate()
            .filter(|(_, job)| *job.get_task() == Some(task))
            .map(|(id, _)| id)
            .collect()
    }

    pub fn validate(&self) -> Result<(), InvalidScheduleError> {
        for Job(id, interval) in self.jobs.iter() {
            if let Some(id) = id
                && !self.tasks.contains(id)
            {
                return Err(InvalidScheduleError::JobWithoutTask);
            }

            if interval.validate().is_err() {
                return Err(InvalidScheduleError::InvalidInterval(*interval));
            }

            let hyper_period_as_singular_interval = Interval(
                EndPoint::Closed(self.hyper_period),
                EndPoint::Closed(self.hyper_period),
            );
            if interval.right_of(&hyper_period_as_singular_interval) {
                return Err(InvalidScheduleError::IntervalExceedsHyperPeriod);
            }
        }

        // if there are more than 1 jobs, perform checks on subsequent pairs of jobs
        if self.jobs.len() > 1 {
            let adjacent_interval_pairs = self.jobs.as_slice().windows(2).map(|s| match s {
                [left_job, right_job] => (left_job, right_job),
                _ => unreachable!(),
            });

            for (Job(_, left_interval), Job(_, right_interval)) in adjacent_interval_pairs {
                // interval can not be right of its successor
                if left_interval.right_of(right_interval) {
                    return Err(InvalidScheduleError::InvertedJobOrder);
                }
                // to avoid deadlocks, we only allow an interval to shadow its successor if their right endpoints are equal
                if left_interval.shadows(right_interval) {
                    let Interval(_, l_r) = left_interval;
                    let Interval(_, r_r) = right_interval;
                    if *l_r != *r_r {
                        return Err(InvalidScheduleError::ShadowedSuccessor);
                    }
                }
            }
        }
        Ok(())
    }

    /// Pad non overlapping jobs, and first/last job to 0/hyperperiod.
    /// This is necessary as all time points in the hyperperiod must have at least
    /// one job available for scheduling, to avoid deadlocks.
    pub fn pad_jobs(&mut self) {
        let old_jobs = mem::take(&mut self.jobs);
        for job in old_jobs.into_iter() {
            match self.jobs.last() {
                // padding between first job and zero
                None => match job.get_constraint() {
                    Interval(EndPoint::Closed(l), _) if l == Fraction::ZERO => {
                        self.jobs.push(job);
                    }
                    Interval(_, right_endpoint) => {
                        self.jobs.push(Job(
                            None,
                            Interval(EndPoint::Closed(Fraction::ZERO), right_endpoint),
                        ));
                        self.jobs.push(job);
                    }
                },
                // padding between jobs
                Some(Job(_, head_interval)) if !head_interval.overlaps(&job.get_constraint()) => {
                    self.jobs
                        .push(Job(None, head_interval.bridge(&job.get_constraint())));
                    self.jobs.push(job);
                }
                _ => self.jobs.push(job),
            }
        }
        // padding between last job and hyperperiod end
        if let Some(last_job) = self.jobs.last() {
            match last_job.get_constraint() {
                Interval(_, EndPoint::Closed(l)) if l == self.hyper_period => (),
                Interval(left_endpoint, _) => {
                    self.jobs.push(Job(
                        None,
                        Interval(left_endpoint, EndPoint::Closed(self.hyper_period)),
                    ));
                }
            }
        }
    }

    fn from_c_file(s: &'input str) -> Result<Self, ()> {
        let mut comments: Vec<&'input str> = Vec::new();
        let mut comment_start: Option<usize> = None;
        let s_bytes = s.as_bytes();
        // extract comment bodies
        let mut i = 0;
        while s_bytes.len() - i >= 2 {
            match (s_bytes[i], s_bytes[i + 1], comment_start) {
                (b'/', b'*', None) => {
                    i += 2;
                    comment_start = Some(i);
                }
                (b'*', b'/', None) => {
                    return Err(()); // unexpected closing comment
                }
                (b'*', b'/', Some(j)) => {
                    comments.push(&s[j..i - 1]);
                    i += 2;
                    comment_start = None;
                }
                _ => {
                    i += 1;
                }
            }
        }

        // unclosed comment
        if comment_start.is_some() {
            return Err(());
        }

        let parsed_comments: Vec<Self> = comments
            .into_iter()
            .map(|c| schedule_grammar::ScheduleParser::new().parse(c))
            .filter(|c| c.is_ok())
            .map(|c| c.unwrap())
            .collect();

        if parsed_comments.len() > 1 {
            Err(()) // more than one schedule in the file
        } else if parsed_comments.len() != 1 {
            Err(()) // no schedule found
        } else {
            let mut parsed_comments = parsed_comments;
            Ok(parsed_comments.pop().unwrap()) // return the sole schedule
        }
    }
}

impl<'input> TryFrom<&'input str> for Schedule<'input> {
    type Error = ParseError<usize, Token<'input>, &'input str>;

    fn try_from(s: &'input str) -> Result<Self, Self::Error> {
        let a = schedule_grammar::ScheduleParser::new().parse(&s);
        let b = Self::from_c_file(s);

        match (a, b) {
            (Err(e1), Err(_e2)) => Err(e1),
            (Ok(sched), Err(())) | (Err(_), Ok(sched)) => Ok(sched),
            (Ok(_), Ok(_)) => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn shadows_test() {
        // [0, 1)
        let i = Interval(
            EndPoint::Closed(Fraction::ZERO),
            EndPoint::Open(Fraction::from(1)),
        );

        // shadows itself
        assert!(i.shadows(&i));

        // shadows (0.5, 0.6]
        assert!(i.shadows(&Interval(
            EndPoint::Open(Fraction::from(0.5)),
            EndPoint::Closed(Fraction::from(0.6))
        )));

        // does not shadow (0, 1]
        assert!(!i.shadows(&Interval(
            EndPoint::Open(Fraction::ZERO),
            EndPoint::Closed(Fraction::from(1))
        )));
    }

    #[test]
    fn overlaps_test() {
        // [0,3)
        let i = Interval(
            EndPoint::Closed(Fraction::ZERO),
            EndPoint::Open(Fraction::from(3)),
        );

        // overlaps itself
        assert!(i.overlaps(&i));

        // overlaps [0,1]
        assert!(i.overlaps(&Interval(
            EndPoint::Closed(Fraction::ZERO),
            EndPoint::Closed(Fraction::from(1))
        )));
        // overlaps (1,3]
        assert!(i.overlaps(&Interval(
            EndPoint::Closed(Fraction::ZERO),
            EndPoint::Closed(Fraction::from(1))
        )));
        // overlaps [2,4)
        assert!(i.overlaps(&Interval(
            EndPoint::Closed(Fraction::from(2)),
            EndPoint::Open(Fraction::from(4))
        )));
        // overlaps [0,0]
        assert!(i.overlaps(&Interval(
            EndPoint::Closed(Fraction::ZERO),
            EndPoint::Closed(Fraction::ZERO)
        )));
        // does not overlap [3,3]
        assert!(!i.overlaps(&Interval(
            EndPoint::Closed(Fraction::from(3)),
            EndPoint::Closed(Fraction::from(3))
        )));
        // does not overlap [3,4)
        assert!(!i.overlaps(&Interval(
            EndPoint::Closed(Fraction::from(3)),
            EndPoint::Open(Fraction::from(4))
        )));
        // does not overlap [4,5]
        assert!(!i.overlaps(&Interval(
            EndPoint::Closed(Fraction::from(4)),
            EndPoint::Closed(Fraction::from(5))
        )));
    }

    #[test]
    fn left_of_test() {
        // [0, 1]
        let i = Interval(
            EndPoint::Closed(Fraction::ZERO),
            EndPoint::Closed(Fraction::from(1)),
        );

        // left off (1,2]
        assert!(i.left_of(&Interval(
            EndPoint::Open(Fraction::from(1)),
            EndPoint::Closed(Fraction::from(2))
        )));
        // left off (2,3)
        assert!(i.left_of(&Interval(
            EndPoint::Open(Fraction::from(2)),
            EndPoint::Open(Fraction::from(3))
        )));
        // not left off [1,2]
        assert!(!i.left_of(&Interval(
            EndPoint::Closed(Fraction::from(1)),
            EndPoint::Closed(Fraction::from(2))
        )));
    }

    #[test]
    fn right_of_test() {
        // (2, 4]
        let i = Interval(
            EndPoint::Open(Fraction::from(2)),
            EndPoint::Closed(Fraction::from(4)),
        );
        // right off [0,1]
        assert!(i.right_of(&Interval(
            EndPoint::Closed(Fraction::from(0)),
            EndPoint::Closed(Fraction::from(1))
        )));
        // right off [0,2]
        assert!(i.right_of(&Interval(
            EndPoint::Closed(Fraction::from(0)),
            EndPoint::Closed(Fraction::from(2))
        )));
        // not right off [0,3)
        assert!(!i.right_of(&Interval(
            EndPoint::Closed(Fraction::from(0)),
            EndPoint::Closed(Fraction::from(3))
        )));
        // not right off [5,6)
        assert!(!i.right_of(&Interval(
            EndPoint::Closed(Fraction::from(5)),
            EndPoint::Closed(Fraction::from(6))
        )));
    }
}
