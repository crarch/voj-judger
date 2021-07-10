use crate::get_env;
use bson::Document;
use reqwest::{Response,Error};
pub async fn return_result(result:&Document){
    let url=get_env("API_URL")+"/queue";
    let key=get_env("JUDGER_KEY");
    
    let client=reqwest::Client::new();
    let _res=client.post(&url)
        .header("Authorization",key)
        .json(result)
        .send()
        .await;
    
}