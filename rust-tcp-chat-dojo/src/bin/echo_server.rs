use std::env;
use tcp_chat::run_echo_server;

fn main() {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:7878".to_string());
    println!("Echo server starting on {}", addr);

    if let Err(e) = run_echo_server(&addr) {
        eprintln!("Server error: {}", e);
    }
}
