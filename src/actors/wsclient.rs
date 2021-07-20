use actix::prelude::*;
use super::Master;
use super::WsDisconnect;
use super::message::*;

pub struct WsClient{
    pub framed:Option<SinkWrite<Message, SplitSink<Framed<BoxedSocket, Codec>, Message>>>,
    pub master_addr:Addr<Master>
}
    
impl WsClient{
    // pub fn new()->WsClient{
    //     WsClient{
    //         framed:None
    //     }
    // 
    // }
}
    
        

impl Actor for WsClient {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        self.master_addr.do_send(WsConnect(ctx.address()));
    }
    
}
        


impl StreamHandler<Result<Frame, WsProtocolError>> for WsClient {
    fn handle(&mut self, msg: Result<Frame, WsProtocolError>, _: &mut Context<Self>) {
        if let Ok(msg)=msg{
            
            
            match msg{
                Frame::Ping(text)=>{
                    if let Some(ref mut framed)=self.framed{
                        let _result=framed.write(Message::Ping(text));
                    }
                },
                Frame::Pong(_text)=>{
                    // if let Some(ref mut framed)=self.framed{
                    //     let result=framed.write(Message::Ping(text));
                    // }
                },
                
                Frame::Text(job)=>{
                    // self.master_addr.do_send(JudgeJob(String::from(job.split())));
                    let job:Job=serde_json::from_slice(&job).unwrap();
                    println!("{:?}",job);
                },
                
                _=>(),
            }
        }
            
    }

    fn started(&mut self, _ctx: &mut Context<Self>) {
        println!("Connected");
    }

    fn finished(&mut self, ctx: &mut Context<Self>) {
        self.master_addr.do_send(WsDisconnect);
        println!("Server disconnected");
        ctx.stop()
    }
}

impl actix::io::WriteHandler<WsProtocolError> for WsClient {}

use actix::StreamHandler;



use actix::io::SinkWrite;

use actix_codec::Framed;
use awc::{
    error::WsProtocolError,
    ws::{Codec, Frame, Message},
    BoxedSocket
};

use futures::stream::{SplitSink};
use bson::oid::ObjectId;
use serde::{Deserialize,Serialize};



#[derive(Debug,Serialize,Deserialize)]
struct Job{
    _id:ObjectId,
    question_id:u32,
    update:u32,
    user_id:u32,
    code:String,
    submit_time:u32,
}