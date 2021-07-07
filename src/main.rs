use std::collections::HashMap;

mod fetch_question;
mod timestamp;
mod env;
mod judge;


fn main(){
    // fetch_question::fetch_question_by_id(100100);
    judge::judge("60e48f3f0013e0fe00ef8acd",100100);
}

