
use serde::{Deserialize,Serialize};
use std::fs::{self};

use std::time::UNIX_EPOCH;
use crate::env::get_env;
use crate::fetch_testbench::fetch_testbench_by_id;
use std::process::Command;


pub fn fetch_job()->Option<(String,u32)>{
    let key=get_env("JUDGER_KEY");
    let id=get_env("JUDGER_ID");
    
    let url=get_env("API_URL")+"/queue";
    let client=reqwest::blocking::Client::new();
    
    let job;
    
    if let Ok(response) = client.get(url)
        .header("Authorization",key)
        .header("JudgerID",id)
        .send(){
        if let Ok(body)=response.json::<Job>(){
            job=body;
        }else{
        //empty queue
        return None;
        }
    }else{
        return None;
    }
        
    println!("{:?}",&job);
    
    let question_id=job.question_id;
    let update=job.update;
    let code=job.code;
    let job_id=job._id.to_hex();
    let testbench_folder=get_env("JUDGER_HOME")+"/testbenches/"+&question_id.to_string();
    
    let job_dir=get_env("JUDGER_HOME")+"/jobs/"+&job_id;
    
    let mut fetch=true;
    if let Ok(metadata)=fs::metadata(testbench_folder.clone()+"/0"){
        if let Ok(time)=metadata.modified(){
            if let Ok(time)=time.duration_since(UNIX_EPOCH){
                if update<time.as_millis() as u32 {
                    fetch=false;
                }
            }
        }
    }
    
    if fetch {
        fetch_testbench_by_id(question_id);
    }
    
    //cmd:mkdir -p job_dir
    let mut mkdir=Command::new("mkdir");
    mkdir.arg("-p");
    mkdir.arg(&job_dir);
    mkdir.output().unwrap();    
    
    fs::write(&(job_dir+"/code"),code).unwrap();
    
    return Some((job_id,question_id));
    
}


use bson::oid::ObjectId;

#[derive(Debug,Serialize,Deserialize)]
struct Job{
    _id:ObjectId,
    question_id:u32,
    update:u32,
    code:String,
}
