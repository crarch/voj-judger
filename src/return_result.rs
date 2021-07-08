use crate::get_env;
use bson::Document;

pub fn return_result(result:&Document){
    let url=get_env("API_URL")+"/queue";
    let key=get_env("JUDGER_KEY");
    
    let client=reqwest::blocking::Client::new();
    
    let res=client.post(&url)
        .header("Authorization",key)
        .json(result)
        .send().unwrap();
    
    println!("{}",result);
}