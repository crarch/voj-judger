use actix::prelude::*;
use super::Master;
use super::WsDisconnect;
use super::message::*;

use std::time::{Duration, Instant};

pub struct WsClient{
    pub framed:SinkWrite<Message, SplitSink<Framed<BoxedSocket, Codec>, Message>>,
    pub master_addr:Addr<Master>,
    pub hb:Instant,
}

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(2);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(5);
    
impl WsClient{
    fn hb(&self, ctx: &mut Context<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                ctx.stop();
                return;
            }
                
        });
    }
}
    
        

impl Actor for WsClient {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        self.hb(ctx);
        
        self.master_addr.do_send(WsConnect(ctx.address()));
    }
    
    fn stopping(&mut self,_ctx:&mut Self::Context)->Running{
        println!("Server disconnected");
        self.master_addr.do_send(WsDisconnect);
        Running::Stop
    }
    
}

impl Handler<JudgeResult> for WsClient{
    type Result=();
    
    fn handle(&mut self,result:JudgeResult,_ctx:&mut Self::Context){
        let JudgeResult(result)=result;
        let result=serde_json::to_string(&result).unwrap();
        let result=bytestring::ByteString::from(result);
        let _=self.framed.write(Message::Text(result));
    }
    
    
}
        


impl StreamHandler<Result<Frame, WsProtocolError>> for WsClient {
    fn handle(&mut self, msg: Result<Frame, WsProtocolError>, _: &mut Context<Self>) {
        if let Ok(msg)=msg{
            
            
            match msg{
                Frame::Ping(text)=>{
                    self.hb=Instant::now();
                    let _=self.framed.write(Message::Ping(text));
                },
                Frame::Pong(_text)=>{
                },
                
                Frame::Text(job)=>{
                    let job:Job=serde_json::from_slice(&job).unwrap();
                    let _=self.master_addr.do_send(JudgeJob(job));
                },
                
                _=>(),
            }
        }
            
    }

    fn started(&mut self, _ctx: &mut Context<Self>) {
        println!("Connected");
    }

    fn finished(&mut self, ctx: &mut Context<Self>) {
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


use super::Job;