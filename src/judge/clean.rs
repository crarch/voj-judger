
use std::process::Command;


pub fn clean_dir(job_dir:&str){
    let mut rm=Command::new("rm");
    rm.arg(format!("{}",&job_dir));
    rm.arg("-r");
    rm.output().unwrap();
}

