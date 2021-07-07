use bson::Document;
use serde::{Deserialize,Serialize};
use std::fs::{self,File};
use std::io::prelude::*;
use std::time::UNIX_EPOCH;
use crate::env::get_env;
use std::process::Command;

pub fn fetch_job(){
    let key=get_env("JUDGER_KEY");
    let id=get_env("JUDGER_ID");
    
    let url=get_env("API_URL")+"/queue";
    let client=reqwest::blocking::Client::new();
    let res = client.get(url)
        .header("Authorization",key)
        .header("JudgerID",id)
        .send()
        .unwrap();
        
    println!("{:?}",res);
        
}
