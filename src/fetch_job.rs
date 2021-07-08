use bson::Document;
use serde::{Deserialize,Serialize};
use std::fs::{self,File};
use std::io::prelude::*;
use std::time::UNIX_EPOCH;
use crate::env::get_env;
use crate::fetch_question::fetch_question_by_id;
use std::process::Command;


pub fn fetch_job(){
    let key=get_env("JUDGER_KEY");
    let id=get_env("JUDGER_ID");
    
    let url=get_env("API_URL")+"/queue";
    let client=reqwest::blocking::Client::new();
    
    let job;
    
    if let Ok(res) = client.get(url)
        .header("Authorization",key)
        .header("JudgerID",id)
        .send()
        .unwrap()
        .json::<Job>(){
            job=res;
    }else{
        //empty queue
        return;
    }
        
    println!("{:?}",&job);
    
    let question_id=job.question_id;
    let update=job.update;
    let code=job.code;
    let job_id=job._id.to_hex();
    let question_folder=get_env("JUDGER_HOME")+"/questions/"+&question_id.to_string();
    
    let job_dir=get_env("JUDGER_HOME")+"/jobs/"+&job_id;
    
    let mut fetch=true;
    if let Ok(metadata)=fs::metadata(question_folder.clone()+"/0"){
        if let Ok(time)=metadata.modified(){
            if let Ok(time)=time.duration_since(UNIX_EPOCH){
                if(update<time.as_millis() as u32){
                    fetch=false;
                }
            }
        }
    }
    
    if(fetch){
        fetch_question_by_id(question_id);
    }
    
    //cmd:mkdir -p job_dir
    let mut mkdir=Command::new("mkdir");
    mkdir.arg("-p");
    mkdir.arg(&job_dir);
    mkdir.output().unwrap();    
    
    fs::write(&(job_dir+"/code"),code).unwrap();
    
    judge(&job_id,question_id);
}

use crate::judge::judge;
use bson::oid::ObjectId;

#[derive(Debug,Serialize,Deserialize)]
struct Job{
    _id:ObjectId,
    question_id:u32,
    update:u32,
    code:String,
}
