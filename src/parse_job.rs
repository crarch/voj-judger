

use serde::{Deserialize,Serialize};
use std::fs::{self};

use std::time::UNIX_EPOCH;
use crate::env::get_env;
use crate::fetch_testbench::fetch_testbench_by_id;
use std::process::Command;



pub fn parse_job(data:&str)->Option<(String,u32,u32)>{
    let job:Job=serde_json::from_str(data).unwrap();
    let question_id=job.question_id;
    let update=job.update;
    let code=job.code;
    let user_id=job.user_id;
    let job_id=job._id.to_hex();
    let testbench_folder=get_env("JUDGER_HOME")+"/testbenches/"+&question_id.to_string();
    
    let job_dir=get_env("JUDGER_HOME")+"/jobs/"+&job_id;
    
    let mut fetch=true;
    if let Ok(metadata)=fs::metadata(testbench_folder.clone()+"/0"){
        if let Ok(time)=metadata.modified(){
            if let Ok(time)=time.duration_since(UNIX_EPOCH){
                if(update<time.as_millis() as u32){
                    fetch=false;
                }
            }
        }
    }
    
    if(fetch){
        fetch_testbench_by_id(question_id);
    }
    
    //cmd:mkdir -p job_dir
    let mut mkdir=Command::new("mkdir");
    mkdir.arg("-p");
    mkdir.arg(&job_dir);
    mkdir.output().unwrap();    
    
    fs::write(&(job_dir+"/code"),code).unwrap();
    return Some((job_id,question_id,user_id));
    
}


use bson::oid::ObjectId;

#[derive(Debug,Serialize,Deserialize)]
struct Job{
    _id:ObjectId,
    update:u32,
    question_id:u32,
    user_id:u32,
    code:String,
    lock_time:u32,
}

    
