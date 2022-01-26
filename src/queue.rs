use crate::JobQueueError;
use std::collections::VecDeque;
use std::sync::Mutex;

pub struct JobQueue {
    //Trade off was Mutex vs Channels
    //From a high level I like the philosophy of Channels which under the hood uses needs locking regardless
    //Since the problem is inherently shared storage, used Mutex directly instead of the Channel abstraction
    queue: Mutex<VecDeque<String>>,
}

impl JobQueue {
    pub fn new() -> Self {
        Self {
            queue: Mutex::from(VecDeque::new()),
        }
    }

    pub fn enqueue(&self, id: String) -> Result<String, JobQueueError> {
        let mut queue = self.queue.lock().unwrap(); //map to an error here
        queue.push_back(id.clone()); // Check to see if push will be successful? Should we bound the queue?
        Ok(id)
    }

    pub fn dequeue(&self) -> Result<Option<String>, JobQueueError> {
        let mut queue = self.queue.lock().unwrap(); //map to an error here
        Ok(queue.pop_front())
    }
}

#[cfg(test)]
mod tests {
    use crate::queue::JobQueue;
    use crate::JobQueueError;

    #[test]
    pub fn job_queue_should_enqueue_and_dequeue() -> Result<(), JobQueueError> {
        let queue = JobQueue::new();
        let expected: Option<String> = Some(String::from("123"));

        queue.enqueue(expected.clone().unwrap())?;

        let actual = queue.dequeue()?;

        assert_eq!(actual, expected);
        Ok(())
    }
}
