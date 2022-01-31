use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum JobQueueError {
    #[error("Internal Error")]
    InternalError,
    #[allow(dead_code)]
    #[error("Not Implemented")]
    NotImplemented,
    #[error("Not Found")]
    NotFound,
    #[error("Unauthorized")]
    Unauthorized,
}

#[derive(Serialize)]
pub struct JobIdentifier {
    pub id: String,
}

#[derive(Clone, Eq, PartialEq, Deserialize, Debug, Serialize)]
pub struct JobRequest {
    #[serde(rename = "type")]
    pub job_type: JobType,
}

#[derive(Clone, Eq, PartialEq, Deserialize, Debug, Serialize)]
pub struct Job {
    pub id: String,
    #[serde(rename = "type")]
    pub job_type: JobType,
    #[serde(rename = "status")]
    pub job_status: JobStatus,
    pub job_implementer: Option<String>,
}

#[derive(Clone, Eq, PartialEq, Deserialize, Debug, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum JobType {
    TimeCritical,
    NotTimeCritical,
}

#[derive(Clone, Eq, PartialEq, Deserialize, Debug, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum JobStatus {
    Queued,
    InProgress,
    Concluded,
}
