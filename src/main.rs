#![allow(unused_assignments,dead_code,unused_must_use,unused_parens)]


mod fetch_testbench;
mod fetch_job;
mod timestamp;
mod env;
mod judge;
mod parse;
mod worker;
mod return_result;
mod clean;
mod actors;

pub use fetch_job::fetch_job;
pub use judge::judge; 
pub use env::get_env;
pub use return_result::return_result;
pub use clean::clean_dir;

use actix::Actor;
use actors::WsClient;
use tokio_tungstenite::connect_async;
use futures::{StreamExt};



#[actix::main]
async fn main(){
    
    let ws_url="ws".to_string()+&get_env("API_URL")[4..]+"/websocket";
    let judger_key=get_env("JUDGER_KEY");
    
    let header=httparse::Header{
        name:"Authorization",
        value:&judger_key.as_bytes(),
    };
    
    let mut headers = vec!(header);
    let request=httparse::Request{
        method:Some("GET"),
        path:Some(&ws_url),
        version:Some(1),
        headers:&mut headers,
    };


    let (ws_stream,_)=connect_async(request).await.unwrap();
    
    let (tx,_rx)=ws_stream.split();
    
    // tx.send(Message::Text("hihi".to_string())).await;
    
    let _addr=WsClient::create(|_ctx|{
        // WsClient::add_stream(FramedRead::new(rx,codec::ClientCodec),ctx);
        WsClient{
            framed:tx
        }
    });
    
    loop{}
}








mod codec;