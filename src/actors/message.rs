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
pub struct JudgeJob(pub String);

#[derive(Message)]
#[rtype(result="()")]
pub struct JudgeResult(pub String);

#[derive(Message)]
#[rtype(result="()")]
pub struct WsDisconnect;

#[derive(Message)]
#[rtype(result="()")]
pub struct WsConnect(pub Addr<WsClient>);

#[derive(Message)]
#[rtype(result="()")]
pub struct SpawnWsClient;
