use actix::prelude::*;

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
pub struct JudgeResult(pub Job);

#[derive(Message)]
#[rtype(result="()")]
pub struct WsDisconnect;

#[derive(Message)]
#[rtype(result="()")]
pub struct WsConnect(pub Addr<WsClient>);

#[derive(Message)]
#[rtype(result="()")]
pub struct SpawnWsClient;


use super::Job;

