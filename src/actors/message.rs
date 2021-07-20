use actix::prelude::*;
use uuid::Uuid;
use super::*;

#[derive(Message)]
#[rtype(result="()")]
pub struct Connect{
    pub addr:Recipient<JudgeJob>,
    pub self_id:Uuid,
}

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
    _id:ObjectId,
    question_id:u32,
    update:u32,
    user_id:u32,
    code:String,
    submit_time:u32,
}

use bson::oid::ObjectId;
use bson::document::Document;
use serde::{Deserialize,Serialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct JobResult{
    _id:ObjectId,
    success:bool,
    user_id:u32,
    code:String,
    question_id:u32,
    submit_time:u32,
    test_bench:Document,
}