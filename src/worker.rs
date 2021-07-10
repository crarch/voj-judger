use crate::fetch_job;
use crate::judge;
use crate::return_result;

pub async fn start(job_id:String,question_id:u32,user_id:u32){
    let result=judge(&job_id,question_id,user_id).unwrap();
    return_result(&result).await;
}    