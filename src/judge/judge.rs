use crate::env::get_env;
use std::path::Path;



use bson::Document;
use bson::doc;

use super::clean_dir;
use super::parse;
use crate::actors::Job;
use std::fs::{self};


pub fn judge(mut job:Job)->Job{
    
    let testbench_id=job.question_id;
    let test_bench_dir=get_env("JUDGER_HOME")
        +"/testbenches/"
        +&(testbench_id.to_string());
    
    let test_bench_dir=Path::new(&test_bench_dir);    
    if(!test_bench_dir.exists()){
        job.success=false;
        job.test_bench=doc!{
            "error":"no such question"
        };
        return job;
    }
    
        
    let job_id=job._id.to_hex();
    
    let job_dir=get_env("JUDGER_HOME")+"/jobs/"+&job_id;
    let mut mkdir=Command::new("mkdir");
    mkdir.arg("-p");
    mkdir.arg(&job_dir);
    mkdir.output().unwrap();    
    fs::write(&(job_dir.clone()+"/code"),&job.code).unwrap();
    
    
    
    //make vcd folder
    let mut make_dir_vcd=Command::new(
        "mkdir"
    );
    make_dir_vcd.arg("-p");
    make_dir_vcd.arg(format!("{}/vcd",&job_dir));
    make_dir_vcd.output().unwrap();
    
        
    
    let test_points=test_bench_dir.read_dir().unwrap();
    
    let mut success=true;
    
    let mut test_benches=doc!{};
    
    for test_point in test_points{
        
        let id=test_point
            .as_ref()
            .unwrap()
            .path()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
            
        let id=id[..(id.len()-2)]
            .parse::<u32>()
            .unwrap();
        
        let test_point=test_point
            .unwrap()
            .path()
            .into_os_string()
            .into_string()
            .unwrap();
            
        match judge_test_point(&test_point,&job_dir,id){
            None=>{
                test_benches.insert(id.to_string(),"");
            },
            Some(wave)=>{
                success=false;
                if wave.contains_key("compile_error") {
                    test_benches.insert(
                        "compile_error",
                        wave.get_str("compile_error").unwrap().to_string()
                    );
                    break;
                }else{
                    test_benches.insert(id.to_string(),wave);
                }
            },
        }    
        
    }
    
    if(get_env("DEBUG")!="true"){
        clean_dir(&job_dir);
    }
    
    
    job.success=success;
    job.test_bench=test_benches;
    
    job
    
}

use std::process::Command;


fn judge_test_point(test_point:&str,job_dir:&str,id:u32)->Option<Document>{
    //todo:Result<(),()>
    
    //cmd:iverilog code tb
    // let mut test=Command::new("iverilog");
    let mut test=Command::new("timeout");
    test.arg("2");
    test.arg("iverilog");
    test.arg("-Tmax");
    test.arg("-g2012");
    test.arg(format!("{}/code",&job_dir));
    test.arg(test_point);
    test.arg("-o");
    test.arg(format!("{}/a.out",&job_dir));
    
    
    let output=test.output().unwrap();
    
    if let Some(code)=output.status.code(){
        if(code==124){
            let result=doc!{
                "compile_timeout":"timeout",
            };
            return Some(result);
        }
        //return time out
    }
    //todo:pass compile error message
    let std_err=output.stderr;
    let std_err=String::from_utf8(std_err).unwrap();
    if std_err!="" {
        let result=doc!{
            "compile_error":std_err,
        };
        return Some(result);
    }
    
    
    //cmd:./a.out
    // let mut run=Command::new(
    //     format!("{}/a.out",&job_dir)
    // );
    let mut run=Command::new("timeout");
    run.arg("2");
    run.arg(format!("{}/a.out",&job_dir));
    run.current_dir(job_dir);
    let output=run.output().unwrap();
    
    if let Some(code)=output.status.code(){
        if(code==124){
            let result=doc!{
                "run_timeout":"timeout",
            };
            return Some(result);
        }
        //return time out
    }
    
    let vcd=format!("{}/vcd/{}.vcd",job_dir,id);
    
    //cmd cp *.vcd vcd/{id}.vcd
    let mut mv=Command::new("mv");
    mv.arg(format!("{}/dump.vcd",job_dir));
    mv.arg(&vcd);
    mv.current_dir(job_dir);
    mv.output().unwrap();
        
    let mut mv=Command::new("mv");
    mv.arg(format!("{}/wave.vcd",job_dir));
    mv.arg(&vcd);
    mv.current_dir(job_dir);
    mv.output().unwrap();
    
    // parse vcd
    parse(&vcd)
    //clean
    //cmd:rm {id}.vcd
    
}