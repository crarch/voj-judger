#![allow(unused_assignments,dead_code,unused_must_use,unused_parens)]


mod fetch_testbench;
mod parse_job;
mod timestamp;
mod env;
mod judge;
mod parse;
mod worker;
mod return_result;
mod clean;
mod thread_pool;

pub use parse_job::parse_job;
pub use judge::judge; 
pub use env::get_env;
pub use return_result::return_result;
pub use clean::clean_dir;


use websocket_lite::{ClientBuilder,Message,Opcode};

fn main(){

    let workers=get_env("WORKERS").parse::<usize>().unwrap();

    let pool=thread_pool::ThreadPool::new(workers);

    let ws_url="ws".to_string()+&get_env("API_URL")[4..]+"/websocket";

    let mut client=ClientBuilder::new(&ws_url).unwrap().connect().unwrap();


    while let Ok(Some(message))=client.receive(){
        match message.opcode(){
            Opcode::Text=>{
                let data=message.as_text().unwrap();

                if let Some((job_id,question_id,user_id))=parse_job(data){
                    println!("judging {}",&job_id);
                    pool.execute(move||{
                        worker::start(job_id,question_id,user_id);
                    });
                }


            },

            Opcode::Ping => client.send(Message::pong(message.into_data())).unwrap(),

            _=>(),

        }
    }

}


