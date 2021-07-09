use crate::fetch_job;
use crate::judge;
use crate::return_result;

pub fn start(job_id:String,question_id:u32){
    let result=judge(&job_id,question_id).unwrap();
    return_result(&result);
}    