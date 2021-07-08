
pub fn get_env(env_name:&str)->String{
    dotenv::dotenv().ok();
    std::env::var(env_name).expect(&(format!("missing environment variable {}",env_name))[..])
}

