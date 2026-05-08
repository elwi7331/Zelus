use crate::schedule::{Schedule, Task};

pub trait C {
    fn as_code(&self) -> String;
}

struct Scheduler {
    number_of_states: usize,
}

impl From<&Schedule<'_>> for Scheduler {
    fn from(schedule: &Schedule) -> Self {
        let number_of_states = schedule.get_number_of_jobs();
        Self { number_of_states }
    }
}

impl C for Scheduler {
    fn as_code(&self) -> String {
        format!(
            "void scheduler() {{\n    if ( scheduler_state == {} ) {{\n        scheduler_state = 0;\n    }} else {{\n        scheduler_state += 1;\n    }}\n}}",
            self.number_of_states - 1
        )
    }
}

/// associates task (c function) with job ids (states of the scheduler)
struct JobCondition<'input> {
    task_name: &'input str,
    job_ids: Vec<usize>,
}

impl<'input> JobCondition<'input> {
    fn from_task(task: &'input Task<'input>, schedule: &Schedule<'input>) -> Self {
        let job_ids = schedule.task_to_job_ids(*task);
        let task_name = task.get_name();
        Self { task_name, job_ids }
    }
}

impl C for JobCondition<'_> {
    /// indicates under which conditions (job ids/scheduler states) a task(c function) should be executed
    fn as_code(&self) -> String {
        let conditions = self
            .job_ids
            .iter()
            .map(|id| format!("scheduler_state == {}", id))
            .collect::<Vec<String>>()
            .join(" || ");
        format!("if ( {} ) {{ /* {} body */ }}", conditions, self.task_name)
    }
}

pub struct CAnnotations<'input> {
    scheduler: Scheduler,
    job_conditions: Vec<JobCondition<'input>>,
}

impl<'input> From<&'input Schedule<'input>> for CAnnotations<'input> {
    fn from(schedule: &'input Schedule) -> Self {
        let scheduler = Scheduler::from(schedule);
        let tasks = schedule.get_tasks();
        let job_conditions = tasks
            .into_iter()
            .map(|t| JobCondition::from_task(t, schedule))
            .collect();

        Self {
            scheduler,
            job_conditions,
        }
    }
}

impl C for &[JobCondition<'_>] {
    fn as_code(&self) -> String {
        self.iter()
            .map(|jc| jc.as_code())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl C for CAnnotations<'_> {
    fn as_code(&self) -> String {
        format!(
            "// put:\nunsigned int scheduler_state = 0;\n// at the top of the file (it is recommended to constrain it in the init function also)\n// and add function:\n{}\n// which should run at the bottom of the \"main loop\".\n// Annotate the top of all task bodies with:\n{}",
            self.scheduler.as_code(),
            self.job_conditions.as_slice().as_code()
        )
    }
}
