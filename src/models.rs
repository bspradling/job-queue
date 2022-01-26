use serde::Serialize;

#[derive(Serialize)]
pub struct JobIdentifier {
    pub id: String,
}

#[derive(Serialize)]
pub struct Job {
    pub id: String,
    #[serde(rename = "type")]
    pub job_type: JobType,
    #[serde(rename = "status")]
    pub job_status: JobStatus,
}

#[derive(Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum JobType {
    TimeCritical,
    NotTimeCritical,
}

#[derive(Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum JobStatus {
    Queued,
    InProgress,
    Concluded,
}
