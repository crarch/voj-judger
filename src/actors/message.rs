use actix::prelude::*;
use uuid::Uuid;

#[derive(Message)]
#[rtype(result="()")]
pub struct Connect{
    pub addr:Recipient<JudgeJob>,
    pub self_id:Uuid,
}

#[derive(Message)]
#[rtype(result="()")]
pub struct JudgeJob(pub String);

#[derive(Message)]
#[rtype(result="()")]
pub struct JudgeResult(pub String);

