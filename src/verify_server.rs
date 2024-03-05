use std::env;
pub fn verify_server() {
    let paths: String = env::var("path").unwrap();
    println!("{:?}", paths);
}
