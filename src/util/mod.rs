use std::env;

pub mod tracer;

pub fn check_env() {
    let args: Vec<String> = env::args().collect();
    let env_file = args
        .iter()
        .skip(1)
        .next()
        .cloned()
        .unwrap_or(String::from(".env"));
    println!("currently using {}", env_file);
    dotenvy::from_filename(env_file).ok();
}
