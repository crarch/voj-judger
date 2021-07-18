#![allow(unused_assignments,dead_code,unused_must_use,unused_parens)]

mod fetch_testbench;
mod parse_job;
mod timestamp;
mod env;
mod judge;
mod parse;
mod worker;
mod return_result;
mod clean;

pub use parse_job::parse_job;
pub use judge::judge; 
pub use env::get_env;
pub use return_result::return_result;
pub use clean::clean_dir;


use tokio;

use websocket_lite::{ClientBuilder,Message,Opcode};
use futures::stream::StreamExt;
use futures::SinkExt;

use bson::Bson;
use serde_json::Value;

#[tokio::main]
async fn main(){
    
    let judger_key=get_env("JUDGER_KEY");
    
    let ws_url="ws".to_string()+&get_env("API_URL")[4..]+"/websocket";
    
    
    loop{
        let mut client=ClientBuilder::new(&ws_url)
            .unwrap();
            
        client.add_header("Authorization".to_string(),judger_key.clone());
        
        if let Ok(mut client)=client.async_connect().await{

            println!("WebSocket Connection Established");
            worker::start(client).await;
            
        }else{
            continue;
        }
        
    }
}

