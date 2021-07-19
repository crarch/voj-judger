use actix::prelude::*;
use std::str::FromStr;
use std::time::Duration;
use std::{io, net, thread};

use tokio::io::WriteHalf;
use futures_util::stream::SplitSink;
use tokio_util::codec::Framed;

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

/// Server communication

// impl StreamHandler<> for WsClient {
//     fn handle(
//         &mut self,
//         message: 
//         ctx: &mut Context<Self>,
//     ) {
//         // match message.opcode(){
//         //     // Opcode::Ping=>self.framed.send(Message::pong(message.into_data())).unwrap(),
//         //     // Opcode::Pong=>self.framed.send(Message::ping(message.into_data())).unwrap(),
//         //     _=>(),
//         // }
//         println!("ok");
// 
//     }
// }
