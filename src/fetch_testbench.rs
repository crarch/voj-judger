use bson::Document;
use serde::{Deserialize,Serialize};
use std::fs::{self};


use crate::env::get_env;
use std::process::Command;

#[derive(Debug,Serialize,Deserialize)]
struct TestBench{
    _id:u32,
    update:u32,
    test_bench:Document,
}

pub fn fetch_testbench_by_id(testbench_id:u32)->Result<(),()>{
    let testbench_folder=get_env("JUDGER_HOME")+"/testbenches/"+&testbench_id.to_string();
    
    let url=get_env("API_URL")+"/testbench/"+&(testbench_id.to_string());
    let resp = reqwest::blocking::get(url)
        .unwrap()
        .json::<TestBench>()
        .unwrap();
    
    //create new folder
    let mut mkdir=Command::new("mkdir");
    mkdir.arg("-p");
    mkdir.arg(&testbench_folder);
    mkdir.output().unwrap();
    
    
    let test_bench=resp.test_bench;    
    
    for iter in test_bench.iter(){
        let (test_bench_name,test_bench_content)=iter;
        let file=testbench_folder.clone()+"/"+&test_bench_name;
        let test_bench_content=test_bench_content.as_str().unwrap();
        fs::write(&file,test_bench_content).unwrap();
    }    
    
    Ok(())
}
    
