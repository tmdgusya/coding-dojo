use std::env;
use tcp_chat::run_chat_server;

fn main() {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:7879".to_string());
    println!("Chat server starting on {}", addr);

    if let Err(e) = run_chat_server(&addr) {
        eprintln!("Server error: {}", e);
    }
}
