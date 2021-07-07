use std::collections::HashMap;

mod fetch_question;
mod timestamp;
mod env;


fn main(){
    fetch_question::fetch_question_by_id(100100);
}

