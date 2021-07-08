use crate::env::get_env;
use std::path::Path;
use serde::{Deserialize,Serialize};
use bson::Document;
use bson::doc;

use crate::clean_dir;
use crate::parse::parse;

pub fn judge(job_id:&str,testbench_id:u32)->Result<Document,()>{
    //todo:Result<(),()>
    let test_bench_dir=get_env("JUDGER_HOME")
        +"/testbenches/"
        +&(testbench_id.to_string());
    
    let job_dir=get_env("JUDGER_HOME")
        +"/jobs/"
        +job_id;
    
    //rm a.out
    let mut rm=Command::new("rm");
    rm.arg(format!("{}/a.out",&job_dir));
    rm.output().unwrap();
    
    
    //make vcd folder
    let mut make_dir_vcd=Command::new(
        "mkdir"
    );
    make_dir_vcd.arg("-p");
    make_dir_vcd.arg(format!("{}/vcd",&job_dir));
    make_dir_vcd.output().unwrap();
    
    let test_bench_dir=Path::new(&test_bench_dir);    
    
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
                if(wave.contains_key("compile_error")){
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
    
    let result=doc!{
        "_id":job_id.to_string(),
        "success":success,
        "test_bench":test_benches,
    };
    
    clean_dir(&job_dir);
    
    Ok(result)
}

use std::process::Command;
use bson::oid::ObjectId;

fn judge_test_point(test_point:&str,job_dir:&str,id:u32)->Option<Document>{
    //todo:Result<(),()>
    
    //cmd:iverilog code tb
    let mut test=Command::new("iverilog");
    test.arg(format!("{}/code",&job_dir));
    test.arg(test_point);
    test.arg("-o");
    test.arg(format!("{}/a.out",&job_dir));
    //todo:pass compile error message
    let std_err=test.output().unwrap().stderr;
    let std_err=String::from_utf8(std_err).unwrap();
    if(std_err!=""){
        let result=doc!{
            "compile_error":std_err,
        };
        return Some(result);
    }
    
    
    //cmd:./a.out
    let mut run=Command::new(
        format!("{}/a.out",&job_dir)
    );
    run.current_dir(job_dir);
    run.output().unwrap();
    
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