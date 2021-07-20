use serde::{Deserialize,Serialize};
use std::fs::{self};

use std::time::UNIX_EPOCH;
use crate::env::get_env;
use crate::fetch_testbench::fetch_testbench_by_id;
use std::process::Command;


pub fn fetch_job()->Option<(String,u32,u32)>{
    
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
    return Some((job_id,question_id,user_id));
    
}


use bson::oid::ObjectId;

#[derive(Debug,Serialize,Deserialize)]
struct Job{
    _id:ObjectId,
    question_id:u32,
    update:u32,
    user_id:u32,
    code:String,
}
