use std::env;
use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::thread;

fn main() -> io::Result<()> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:7879".to_string());
    println!("Connecting to {}...", addr);

    let stream = TcpStream::connect(&addr)?;
    println!("Connected! Type messages (Ctrl+C to quit):");

    let reader_stream = stream.try_clone()?;
    thread::spawn(move || {
        let reader = BufReader::new(reader_stream);
        for line in reader.lines() {
            match line {
                Ok(msg) => println!("{}", msg),
                Err(_) => break,
            }
        }
    });

    let mut writer = stream;
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let msg = line?;
        writeln!(writer, "{}", msg)?;
        writer.flush()?;
    }

    Ok(())
}
