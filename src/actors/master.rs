use actix::{Actor};

use actix::prelude::{Context,Handler};
use actix::Addr;

use super::message::*;
use super::wsclient::*;


use std::time::Duration;
use std::thread;

use actix::StreamHandler;
use actix::AsyncContext;
use awc::ClientBuilder;
use actix::io::SinkWrite;
use super::WsDisconnect;
use super::Worker;
use futures::StreamExt;
use std::time::{Duration, Instant};

use crate::env::get_env;

pub struct Master{
    workers:Vec<Addr<Worker>>,
    iter:usize,
    workers_count:usize,
    wsclient_addr:Option<Addr<WsClient>>
}


impl Master{
    pub fn new()->Master{
        Master{
            workers:Vec::new(),
            workers_count:0,
            iter:0,
            wsclient_addr:None
        }
    }
}

impl Actor for Master{
    type Context=Context<Self>;
    
    fn started(&mut self, ctx: &mut Context<Self>) {
        ctx.address().do_send(SpawnWsClient);
    }

}

impl Handler<JudgeJob> for Master{
    type Result=();
    
    fn handle(&mut self,
        job:JudgeJob,
        _ctx:&mut Context<Self>
    )->Self::Result{
        self.send_job(job);
    }
} 

impl Handler<WorkerConnect> for Master{
    type Result=();
    
    fn handle(&mut self,
        worker:WorkerConnect,
        _ctx:&mut Context<Self>
    )->Self::Result{
        let WorkerConnect(addr)=worker;
        
        self.workers.push(addr);
        self.workers_count+=1;
        
    }
} 

impl Handler<JudgeResult> for Master{
    type Result=();
    
    fn handle(
        &mut self,
        job:JudgeResult,
        _ctx:&mut Context<Self>
    )->Self::Result{
        if let Some(ref mut addr)=self.wsclient_addr{
            addr.do_send(job);
        }
    }
} 
            

impl Master{
    fn send_job(&mut self,job:JudgeJob){
        
        if self.workers_count==0 {
            ()
        }else{
            if self.iter==self.workers_count {
                self.iter=0;
            }
            
            let addr=&self.workers[self.iter];
            
            let _=addr.do_send(job);
            
            self.iter=self.iter+1;
            
        }
        
    }
    
    
}

impl Handler<WsDisconnect> for Master{
    type Result=();
    
    fn handle(
        &mut self,
        _msg:WsDisconnect,
        ctx:&mut Context<Self>
    )->Self::Result{
        ctx.address().do_send(SpawnWsClient);

    }
}


impl Handler<WsConnect> for Master{
    type Result=();
    
    fn handle(
        &mut self,
        addr:WsConnect,
        _ctx:&mut Context<Self>
    )->Self::Result{
        
        
        let WsConnect(addr)=addr;
        self.wsclient_addr=Some(addr);
    }
}


impl Handler<SpawnWsClient> for Master{
    type Result=();
    
    fn handle(
        &mut self,
        _:SpawnWsClient,
        ctx:&mut Context<Self>
    )->Self::Result{
        let master_addr=ctx.address();
        let fut=async move{
            
            let key=get_env("JUDGER_KEY");
            
            let ws_url="ws".to_string()+&get_env("API_URL")[4..]+"/websocket";
            
            if let Ok((_response, framed)) = ClientBuilder::new()
                .header("Authorization",key)
                .max_http_version(awc::http::Version::HTTP_11)
                .finish()
                .ws(ws_url)
                .connect()
                .await{
                let (sink, stream) = framed.split();
                let _addr = WsClient::create(|ctx| {
                    WsClient::add_stream(stream, ctx);
                    WsClient{
                        framed:SinkWrite::new(sink, ctx),
                        master_addr:master_addr
                        hb:Instant::now()
                    }
                });
            }else{
                thread::sleep(Duration::from_millis(2000));
                master_addr.do_send(SpawnWsClient);
            }
        };
            
        let fut = actix::fut::wrap_future::<_, Self>(fut);
        ctx.spawn(fut);
    }
    
}

