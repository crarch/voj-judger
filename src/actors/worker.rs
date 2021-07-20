use super::JudgeJob;

use super::Master;

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
        _job:JudgeJob,
        _ctx:&mut Context<Self>
    ){
        // println!("{:?}",_job);
    }
    
    
}


impl Handler<JudgeJob> for Worker{
    type Result=();
    
    fn handle(&mut self,_job:JudgeJob,_ctx:&mut Self::Context){
        ()
    }
    
}
