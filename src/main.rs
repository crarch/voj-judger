#![allow(unused_assignments,dead_code,unused_must_use,unused_parens)]

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

use tokio::task;
use tokio;
use tokio::time::Duration;

#[tokio::main]
async fn main(){
    
    loop{
        if let Some((job_id,question_id,user_id))=fetch_job().await{
            println!("judging {}",&job_id);
            task::spawn(async move{worker::start(job_id,question_id,user_id).await});
        }else{
            tokio::time::sleep(Duration::from_millis(1000)).await;
        }
    }
}

