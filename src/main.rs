#[macro_use]
extern crate rocket;

use crate::models::{Job, JobIdentifier};
use rocket::http::hyper::StatusCode;
use rocket::http::{ContentType, Status};
use rocket::response::status::{Accepted, NoContent};
use rocket::response::Responder;
use rocket::serde::json::Json;
use rocket::{response, Build, Request, Response, Rocket};
use serde::Serialize;
use thiserror::Error;

mod models;

#[derive(Error, Debug)]
pub enum JobQueueError {
    #[error("Not Implemented")]
    NotImplemented,
    #[error("Internal Error")]
    InternalError,
}

#[post("/enqueue")]
fn enqueue() -> Accepted<Json<JobIdentifier>> {
    Accepted(Some(Json(JobIdentifier {
        id: "123".to_string(),
    })))
}

#[get("/dequeue", format = "application/json")]
fn dequeue() -> Result<Accepted<Json<JobIdentifier>>, JobQueueError> {
    Err(JobQueueError::NotImplemented)
}

#[post("/<job_id>/conclude", format = "application/json")]
fn conclude(job_id: String) -> Result<NoContent, JobQueueError> {
    println!("{}", job_id);
    Err(JobQueueError::NotImplemented)
}

#[get("/<job_id>", format = "application/json")]
fn get_job(job_id: String) -> Result<Json<Job>, JobQueueError> {
    println!("{}", job_id);
    Err(JobQueueError::NotImplemented)
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/jobs", routes![enqueue, dequeue, conclude, get_job])
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
