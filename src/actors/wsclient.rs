use actix::prelude::*;
use std::str::FromStr;
use std::time::Duration;
use std::{io, net, thread};
use tokio::io::WriteHalf;
use futures_util::stream::SplitSink;

use crate::get_env;

use super::JudgeJob;
use tokio_tungstenite::MaybeTlsStream;
use tokio_tungstenite::WebSocketStream;
use futures_util::SinkExt;
pub struct WsClient {
    pub framed:SplitSink<WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>, tokio_tungstenite::tungstenite::Message>,
}
    
struct Ping;

impl Actor for WsClient {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        self.framed.send(Message::Text("hihi".to_string()));
            
    }

    fn stopped(&mut self, _: &mut Context<Self>) {
    }
}
        
use futures_util::StreamExt;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::tungstenite::Error;


impl StreamHandler<Result<tokio_tungstenite::tungstenite::Message, tokio_tungstenite::tungstenite::Error>> for WsClient {
    fn handle(
        &mut self,
        message:Result<tokio_tungstenite::tungstenite::Message, tokio_tungstenite::tungstenite::Error>, 
        ctx: &mut Context<Self>,
    ) {
        if let Ok(message)=message{
            println!("ok");
            // match message{
            // 
            // 
            // 
            // }
        }

        // println!("{:?}",message);
    }
}


