use crate::env::get_env;
use std::path::Path;

pub fn judge(job_id:&str,question_id:u32){
    //todo:Result<(),()>
    let test_bench_dir=get_env("JUDGER_HOME")
        +"/questions/"
        +&(question_id.to_string());
    
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
            
        judge_test_point(&test_point,&job_dir,id);
        
    }
}

use std::process::Command;

fn judge_test_point(test_point:&str,job_dir:&str,id:u32){
    //todo:Result<(),()>
    
    //cmd:iverilog code tb
    let mut test=Command::new("iverilog");
    test.arg(format!("{}/code",&job_dir));
    test.arg(test_point);
    test.arg("-o");
    test.arg(format!("{}/a.out",&job_dir));
    //todo:pass compile error message
    println!("{:?}",test.output().unwrap());
    
    //cmd:./a.out
    let mut run=Command::new(
        format!("{}/a.out",&job_dir)
    );
    run.current_dir(job_dir);
    run.output().unwrap();
    
    
    //cmd cp *.vcd vcd/{id}.vcd
    let mut mv=Command::new("mv");
    mv.arg(format!("{}/dump.vcd",job_dir));
    mv.arg(format!("{}/vcd/{}.vcd",job_dir,id));
    mv.current_dir(job_dir);
    mv.output().unwrap();
        
    let mut mv=Command::new("mv");
    mv.arg(format!("{}/wave.vcd",job_dir));
    mv.arg(format!("{}/vcd/{}.vcd",job_dir,id));
    mv.current_dir(job_dir);
    mv.output().unwrap();
}