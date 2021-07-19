use actix::prelude::*;
use std::str::FromStr;
use std::time::Duration;
use std::{io, net, thread};
use tokio::io::WriteHalf;
use futures_util::stream::SplitSink;

use crate::get_env;

use super::JudgeJob;

pub struct WsClient {
    // pub framed:actix::io::FramedWrite<Message, WriteHalf<Framed<Box<dyn AsyncNetworkStream + Sync + Unpin + std::marker::Send>, Message>>, MessageCodec>, 
}
    
struct Ping;

impl Actor for WsClient {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        
            
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
        // match message.opcode(){
        //     // Opcode::Ping=>self.framed.send(Message::pong(message.into_data())).unwrap(),
        //     // Opcode::Pong=>self.framed.send(Message::ping(message.into_data())).unwrap(),
        //     _=>(),
        // }
        println!("ok");

    }
}
