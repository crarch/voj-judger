use crate::fetch_job;
use crate::judge;
use crate::return_result;
use std::{thread, time};


pub fn start(){
    let sleep_time=time::Duration::from_millis(1000);
    loop{
        if let Some((job_id,question_id))=fetch_job(){
            let result=judge(&job_id,question_id).unwrap();
            return_result(&result);
        }
        thread::sleep(sleep_time);
    }
}    