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
mod actors;

pub use fetch_job::fetch_job;
pub use judge::judge; 
pub use env::get_env;
pub use return_result::return_result;
pub use clean::clean_dir;

fn main(){
    let system=actix::System::new();
    
    system.run();
}

