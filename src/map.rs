use crate::models::Job;
use crate::JobQueueError;
use std::collections::HashMap;
use std::sync::Mutex;

pub struct JobMap {
    //Trade off was Mutex vs Channels
    //From a high level I like the philosophy of Channels which under the hood uses needs locking regardless
    //Since the problem is inherently shared storage, used Mutex directly instead of the Channel abstraction
    map: Mutex<HashMap<String, Job>>,
}

impl JobMap {
    pub fn new() -> Self {
        Self {
            map: Mutex::from(HashMap::<String, Job>::new()),
        }
    }

    pub fn insert(&self, key: &String, value: Job) -> Result<Option<Job>, JobQueueError> {
        let mut map = self.map.lock().unwrap();
        Ok(map.insert(key.to_string(), value))
    }

    pub fn get(&self, key: &String) -> Result<Job, JobQueueError> {
        let map = self.map.lock().unwrap();
        let optional_job = map.get(key);

        match optional_job {
            Some(job) => Ok(job.clone()),
            None => Err(JobQueueError::NotFound),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::map::JobMap;
    use crate::models::{Job, JobStatus, JobType};
    use crate::JobQueueError;

    #[test]
    pub fn job_map_should_insert_and_get() -> Result<(), JobQueueError> {
        let map = JobMap::new();
        let expected: Option<Job> = Some(Job {
            id: "123".to_string(),
            job_type: JobType::TimeCritical,
            job_status: JobStatus::Queued,
            job_implementer: None,
        });

        let job = expected.clone().unwrap();
        map.insert(&job.clone().id, job.clone())?;

        let actual = map.get(&job.id)?;

        assert_eq!(Some(actual), expected);
        Ok(())
    }
}
