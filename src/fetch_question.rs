use bson::Document;
use serde::{Deserialize,Serialize};
use std::fs::{self,File};
use std::io::prelude::*;
use std::time::UNIX_EPOCH;
use crate::env::get_env;
use std::process::Command;

#[derive(Debug,Serialize,Deserialize)]
struct Question{
    _id:u32,
    update:u32,
    test_bench:Document,
}

pub fn fetch_question_by_id(question_id:u32)->Result<(),()>{
    let question_folder=get_env("JUDGER_HOME")+"/questions/"+&question_id.to_string();
    
    let url=get_env("API_URL")+"/question/"+&(question_id.to_string());
    let resp = reqwest::blocking::get(url)
        .unwrap()
        .json::<Question>()
        .unwrap();
    
    //create new folder
    let mut mkdir=Command::new("mkdir");
    mkdir.arg("-p");
    mkdir.arg(&question_folder);
    mkdir.output().unwrap();
    
    
    let test_bench=resp.test_bench;    
    
    for iter in test_bench.iter(){
        let (test_bench_name,test_bench_content)=iter;
        let file=question_folder.clone()+"/"+&test_bench_name;
        let test_bench_content=test_bench_content.as_str().unwrap();
        fs::write(&file,test_bench_content).unwrap();
    }    
    
    Ok(())
}
    
