use super::JudgeJob;
use super::JudgeResult;
use super::Master;
use super::Connect;
use actix::prelude::*;
use uuid::Uuid;

pub struct Worker{
    id:Uuid,
    master_addr:Addr<Master>,
}

impl Actor for Worker{
    type Context=Context<Self>;
    
}

impl Worker{
    pub fn new(
        master_addr:Addr<Master>,
    )->Worker{
        Worker{
            id:Uuid::new_v4(),
            master_addr:master_addr
        }
    }
}


impl Worker{
    
    fn judge(
        &self,
        job:JudgeJob,
        _ctx:&mut Context<Self>
    ){
        ()
    }
    
    
}


impl Handler<JudgeJob> for Worker{
    type Result=();
    
    fn handle(&mut self,job:JudgeJob,_ctx:&mut Self::Context){
        ()
    }
    
}
