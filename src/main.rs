#[macro_use]
extern crate rocket;

use crate::map::JobMap;
use crate::models::{Job, JobIdentifier, JobQueueError, JobRequest, JobStatus};
use crate::queue::JobQueue;
use rocket::http::hyper::StatusCode;
use rocket::http::{ContentType, Status};
use rocket::response::status::Accepted;
use rocket::response::Responder;
use rocket::serde::json::Json;
use rocket::{response, Build, Request, Response, Rocket, State};
use serde::Serialize;
use uuid::Uuid;

mod map;
mod models;
mod queue;

#[post("/enqueue", data = "<job>")]
fn enqueue(
    job: Json<JobRequest>,
    state: &State<JobMap>,
    queue: &State<JobQueue>,
) -> Result<Accepted<Json<JobIdentifier>>, JobQueueError> {
    let job_id = Uuid::new_v4().to_string();

    let job_data = Job {
        id: job_id.clone(),
        job_type: job.job_type.clone(),
        job_status: JobStatus::Queued,
    };

    state.insert(&job_id, job_data)?;

    // TODO Should I make the queue bounded to protect against OOM panic?
    match queue.enqueue(job_id) {
        Ok(id) => {
            println!("Successfully enqueued the message"); //TODO: use log package with levels {DEBUG, INFO, ERROR}
            Ok(Accepted(Some(Json(JobIdentifier { id }))))
        }
        Err(_) => Err(JobQueueError::InternalError),
    }
}

#[get("/dequeue", format = "application/json")]
fn dequeue(state: &State<JobMap>, queue: &State<JobQueue>) -> Result<Json<Job>, JobQueueError> {
    match queue.dequeue()? {
        Some(id) => {
            //ASSUMPTION: The Job should be marked IN_PROGRESS as it is dequeued
            let current = state.get(&id)?;
            let new = Job {
                id: current.id.clone(),
                job_type: current.job_type.clone(),
                job_status: JobStatus::InProgress,
            };
            state.insert(&current.id, new.clone())?;
            Ok(Json(new))
        }
        //ASSUMPTION: If there is nothing to dequeue, return Not Found
        None => Err(JobQueueError::NotFound),
    }
}

#[post("/<job_id>/conclude", format = "application/json")]
fn conclude(job_id: String, state: &State<JobMap>) -> Result<(), JobQueueError> {
    let job = state.get(&job_id)?;

    state.insert(
        &job_id,
        Job {
            id: job.id,
            job_type: job.job_type,
            job_status: JobStatus::Concluded,
        },
    )?;
    Ok(())
}

#[get("/<job_id>", format = "application/json")]
fn get_job(job_id: String, state: &State<JobMap>) -> Result<Json<Job>, JobQueueError> {
    match state.get(&job_id) {
        Ok(job) => Ok(Json(job)),
        Err(error) => Err(error),
    }
}

#[launch]
fn rocket() -> Rocket<Build> {
    let state = JobMap::new();
    let deque = JobQueue::new();

    //ASSUMPTION: When deployed to production, a load balancer is serving HTTPS and terminating SSL
    rocket::build()
        .mount("/jobs", routes![enqueue, dequeue, conclude, get_job])
        .manage(deque)
        .manage(state)
}

impl<'r, 'o: 'r> Responder<'r, 'o> for JobQueueError {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {
        #[derive(Serialize)]
        struct Error {
            code: String,
            message: String,
        }

        match &self {
            JobQueueError::NotImplemented => Response::build_from(
                Json(Error {
                    code: StatusCode::NOT_IMPLEMENTED.to_string(),
                    message: "Sorry, this API hasn't been implemented yet!".to_string(),
                })
                .respond_to(req)
                .unwrap(),
            )
            .status(Status::NotImplemented)
            .header(ContentType::JSON)
            .ok(),
            JobQueueError::NotFound => Response::build_from(
                Json(Error {
                    code: StatusCode::NOT_FOUND.to_string(),
                    message: "The requested resource was not found".to_string(),
                })
                .respond_to(req)
                .unwrap(),
            )
            .status(Status::NotFound)
            .header(ContentType::JSON)
            .ok(),

            _ => Response::build_from(
                Json(Error {
                    code: StatusCode::INTERNAL_SERVER_ERROR.to_string(),
                    message: "An internal error occurred".to_string(),
                })
                .respond_to(req)
                .unwrap(),
            )
            .status(Status::InternalServerError)
            .header(ContentType::JSON)
            .ok(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    #[test]
    pub fn test_enqueue_returns_successfully() -> Result<(), Box<dyn Error>> {
        //TODO fill in implementation
        Ok(())
    }

    #[test]
    pub fn test_enqueue_returns_422_on_invalid_request_body() -> Result<(), Box<dyn Error>> {
        //TODO fill in implementation
        Ok(())
    }

    #[test]
    pub fn test_enqueue_returns_500_on_transient_inserting_error() -> Result<(), Box<dyn Error>> {
        //TODO fill in implementation
        Ok(())
    }

    #[test]
    pub fn test_concludes_returns_successfully() -> Result<(), Box<dyn Error>> {
        //TODO fill in implementation
        Ok(())
    }

    #[test]
    pub fn test_concludes_returns_404_on_supplied_id_not_found() -> Result<(), Box<dyn Error>> {
        //TODO fill in implementation
        Ok(())
    }

    #[test]
    pub fn test_get_job_returns_successfully() -> Result<(), Box<dyn Error>> {
        //TODO fill in implementation
        Ok(())
    }

    #[test]
    pub fn test_get_job_returns_404_on_supplied_id_not_found() -> Result<(), Box<dyn Error>> {
        //TODO fill in implementation
        Ok(())
    }
}
