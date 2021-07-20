use actix::prelude::*;
use uuid::Uuid;
use super::*;

#[derive(Message)]
#[rtype(result="()")]
pub struct WorkerConnect(
    pub Addr<Worker>
);

#[derive(Message)]
#[rtype(result="()")]
pub struct JudgeJob(pub Job);

#[derive(Message)]
#[rtype(result="()")]
pub struct JudgeResult(pub JobResult);

#[derive(Message)]
#[rtype(result="()")]
pub struct WsDisconnect;

#[derive(Message)]
#[rtype(result="()")]
pub struct WsConnect(pub Addr<WsClient>);

#[derive(Message)]
#[rtype(result="()")]
pub struct SpawnWsClient;

#[derive(Debug,Serialize,Deserialize)]
pub struct Job{
    pub _id:ObjectId,
    pub question_id:u32,
    pub update:u32,
    pub user_id:u32,
    pub code:String,
    pub submit_time:u32,
}

use bson::oid::ObjectId;
use bson::document::Document;
use serde::{Deserialize,Serialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct JobResult{
    pub _id:ObjectId,
    pub success:bool,
    pub user_id:u32,
    pub code:String,
    pub question_id:u32,
    pub submit_time:u32,
    pub test_bench:Document,
}