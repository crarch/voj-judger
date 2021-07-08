use std::collections::HashMap;

mod fetch_question;
mod fetch_job;
mod timestamp;
mod env;
mod judge;
mod parse;


fn main(){
    // fetch_question::fetch_question_by_id(100100,114514);
    // judge::judge("60e48f3f0013e0fe00ef8acd",100100);
    // fetch_job::fetch_job();
    println!("{:?}",parse::parse("/home/hilaolu/judger/jobs/60e668b9008c0c400001b33a/vcd/0.vcd"));
}

