use super::JudgeJob;

use super::Master;
use super::message::*;

use actix::prelude::*;


pub struct Worker{
    master_addr:Addr<Master>
}

impl Actor for Worker{
    type Context=Context<Self>;
    
    fn started(&mut self, _ctx: &mut Self::Context) {
    
    }
    
    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        Running::Stop
    }
    
}

impl Worker{
    pub fn new(addr:Addr<Master>)->Worker{
        Worker{
            master_addr:addr
        }
    }
}


impl Worker{
    
    fn judge(
        &self,
        job:Job,
    )->(){
        let result=crate::judge::judge(job);
        self.master_addr.do_send(JudgeResult(result));
        
        
    }
    
}


impl Handler<JudgeJob> for Worker{
    type Result=();
    
    fn handle(&mut self,job:JudgeJob,_ctx:&mut Self::Context){
        let JudgeJob(job)=job;
        self.judge(job);
    }
    
}
