mod master;
mod message;
mod worker;
mod wsclient;

pub use master::*;
pub use message::*;
pub use worker::*;
pub use wsclient::*;

use serde::{Deserialize,Serialize};
use bson::oid::ObjectId;
use bson::document::Document;
#[derive(Debug,Serialize,Deserialize)]
pub struct Job{
    pub _id:ObjectId,
    pub success:bool,
    pub user_id:u32,
    pub code:String,
    pub question_id:u32,
    pub submit_time:u32,
    pub test_bench:Document,
}
