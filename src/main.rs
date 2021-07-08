#![allow(unused_assignments,dead_code,unused_must_use)]

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

fn main(){
    // fetch_testbench::fetch_testbench_by_id(100200);
    // judge::judge("60e48f3f0013e0fe00ef8acd",100100);
    // fetch_job::fetch_job();
    // println!("{:?}",parse::parse("/home/hilaolu/judger/jobs/60e668b9008c0c400001b33a/vcd/0.vcd"));
    worker::start();
}

