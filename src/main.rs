#![allow(unused_assignments,dead_code,unused_must_use,unused_parens)]
use std::{thread, time};

mod fetch_testbench;
mod fetch_job;
mod timestamp;
mod env;
mod judge;
mod parse;
mod worker;
mod return_result;
mod clean;
mod thread_pool;

pub use fetch_job::fetch_job;
pub use judge::judge; 
pub use env::get_env;
pub use return_result::return_result;
pub use clean::clean_dir;

fn main(){
    let sleep_time=time::Duration::from_millis(1000);
    
    let workers=get_env("WORKERS").parse::<usize>().unwrap();
    
    let pool=thread_pool::ThreadPool::new(workers);
    
    
    loop{
        if let Some((job_id,question_id,user_id))=fetch_job(){
            println!("judging {}",&job_id);
            pool.execute(move||{
                worker::start(job_id,question_id,user_id);
            });
        }else{
            thread::sleep(sleep_time);
        }
    }
}

