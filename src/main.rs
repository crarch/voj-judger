use std::collections::HashMap;

mod fetch_question;
mod fetch_job;
mod timestamp;
mod env;
mod judge;


fn main(){
    // fetch_question::fetch_question_by_id(100100,114514);
    // judge::judge("60e48f3f0013e0fe00ef8acd",100100);
    fetch_job::fetch_job();
}

