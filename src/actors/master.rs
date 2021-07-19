use actix::{Actor};
use uuid::Uuid;
use actix::prelude::{Context,Handler,Recipient};

use super::{Connect,JudgeJob,JudgeResult};

type Socket=Recipient<JudgeJob>;

//todo: connect websocket

pub struct Master{
    workers:Vec<(Uuid,Socket)>,
    iter:usize,
    workers_count:usize
}


impl Default for Master{
    fn default()->Master{
        Master{
            workers:Vec::new(),
            workers_count:0,
            iter:0
        }
    }
}

impl Actor for Master{
    type Context=Context<Self>;
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

impl Handler<Connect> for Master{
    type Result=();
    
    fn handle(
        &mut self,
        msg:Connect,
        _ctx:&mut Context<Self>
    )->Self::Result{
        
        self.workers.push(
            (msg.self_id,msg.addr)
        );
        
    }
}
            

impl Master{
    fn send_job(&mut self,job:JudgeJob){
        if(self.workers_count==0){
            ()
        }else{
            if(self.iter==self.workers_count){
                self.iter=0;
            }
            
            let (_,socket_recipient)=&self.workers[self.iter];
            
            let _=socket_recipient.do_send(job);
            
            self.iter=self.iter+1;
            
        }
        
    }
    
}