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

pub use fetch_job::fetch_job;
pub use judge::judge; 
pub use env::get_env;
pub use return_result::return_result;
pub use clean::clean_dir;

#[tokio::main]
async fn main(){
    let sleep_time=time::Duration::from_millis(1000);
    loop{
        if let Some((job_id,question_id,user_id))=fetch_job().await{
            worker::start(job_id,question_id,user_id).await;
        }else{
            thread::sleep(sleep_time);
        }
    }
}

