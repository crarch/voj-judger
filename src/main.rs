use actix::Actor;

mod env;
mod actors;
mod judge;

use actors::{Master,Worker};

use actix::prelude::*;
fn main(){
    let system=System::new();
    
    let workers=crate::env::get_env("WORKERS").parse::<u32>().unwrap();
    
    let master_addr=system.block_on(async{Master::new().start()});
    for _ in 0..workers{
        let addr=master_addr.clone();
        let a=Arbiter::new();
    
        let exec=async move{
            Worker::new(addr.clone()).start();
            Worker::new(addr.clone()).start();
            Worker::new(addr.clone()).start();
            Worker::new(addr.clone()).start();
            Worker::new(addr.clone()).start();
            Worker::new(addr.clone()).start();
            Worker::new(addr.clone()).start();
            Worker::new(addr).start();
        };
    
        let _=a.spawn(exec);
    }
    
    
    system.run();
    
}