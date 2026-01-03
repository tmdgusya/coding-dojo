use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicU16, Ordering};
use std::thread;
use std::time::Duration;
use tcp_chat::*;

static PORT_COUNTER: AtomicU16 = AtomicU16::new(18000);

fn get_test_port() -> u16 {
    PORT_COUNTER.fetch_add(1, Ordering::SeqCst)
}

fn wait_for_server(addr: &str, max_attempts: u32) -> bool {
    for _ in 0..max_attempts {
        if TcpStream::connect(addr).is_ok() {
            return true;
        }
        thread::sleep(Duration::from_millis(50));
    }
    false
}

// =============================================================================
// Mission 1: Echo Server
// =============================================================================

#[test]
fn mission_1_handle_echo_single_message() {
    let port = get_test_port();
    let addr = format!("127.0.0.1:{}", port);

    let server_addr = addr.clone();
    thread::spawn(move || {
        let listener = TcpListener::bind(&server_addr).unwrap();
        // Handle 2 connections: one for wait_for_server, one for actual test
        for stream in listener.incoming().take(2) {
            if let Ok(stream) = stream {
                let _ = handle_echo_client(stream);
            }
        }
    });

    assert!(wait_for_server(&addr, 20), "Server failed to start");

    let mut stream = TcpStream::connect(&addr).unwrap();
    writeln!(stream, "Hello").unwrap();
    stream.flush().unwrap();

    let mut reader = BufReader::new(&stream);
    let mut response = String::new();
    reader.read_line(&mut response).unwrap();

    assert_eq!(response.trim(), "Hello");
}

#[test]
fn mission_1_handle_echo_multiple_messages() {
    let port = get_test_port();
    let addr = format!("127.0.0.1:{}", port);

    let server_addr = addr.clone();
    thread::spawn(move || {
        let listener = TcpListener::bind(&server_addr).unwrap();
        // Handle 2 connections: one for wait_for_server, one for actual test
        for stream in listener.incoming().take(2) {
            if let Ok(stream) = stream {
                let _ = handle_echo_client(stream);
            }
        }
    });

    assert!(wait_for_server(&addr, 20), "Server failed to start");

    let mut stream = TcpStream::connect(&addr).unwrap();
    let mut reader = BufReader::new(stream.try_clone().unwrap());

    let messages = ["First", "Second", "Third"];
    for msg in messages {
        writeln!(stream, "{}", msg).unwrap();
        stream.flush().unwrap();

        let mut response = String::new();
        reader.read_line(&mut response).unwrap();
        assert_eq!(response.trim(), msg);
    }
}

#[test]
fn mission_1_handle_echo_empty_line_disconnect() {
    let port = get_test_port();
    let addr = format!("127.0.0.1:{}", port);

    let server_addr = addr.clone();
    let handle = thread::spawn(move || {
        let listener = TcpListener::bind(&server_addr).unwrap();
        let mut last_result = Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "No connection",
        ));
        // Handle 2 connections: one for wait_for_server, one for actual test
        for stream in listener.incoming().take(2) {
            if let Ok(stream) = stream {
                last_result = handle_echo_client(stream);
            }
        }
        last_result
    });

    assert!(wait_for_server(&addr, 20), "Server failed to start");

    let mut stream = TcpStream::connect(&addr).unwrap();
    writeln!(stream).unwrap();
    stream.flush().unwrap();
    drop(stream);

    let result = handle.join().unwrap();
    assert!(result.is_ok());
}

// =============================================================================
// Mission 2: Multi-Client Echo Server
// =============================================================================

#[test]
fn mission_2_concurrent_clients() {
    let port = get_test_port();
    let addr = format!("127.0.0.1:{}", port);

    let server_addr = addr.clone();
    thread::spawn(move || {
        let _ = run_multi_client_echo_server(&server_addr);
    });

    assert!(wait_for_server(&addr, 20), "Server failed to start");

    let mut handles = vec![];
    for i in 0..3 {
        let client_addr = addr.clone();
        let handle = thread::spawn(move || {
            let mut stream = TcpStream::connect(&client_addr).unwrap();
            let msg = format!("Client{}", i);

            writeln!(stream, "{}", msg).unwrap();
            stream.flush().unwrap();

            let mut reader = BufReader::new(&stream);
            let mut response = String::new();
            reader.read_line(&mut response).unwrap();

            assert_eq!(response.trim(), msg);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

#[test]
fn mission_2_sequential_clients() {
    let port = get_test_port();
    let addr = format!("127.0.0.1:{}", port);

    let server_addr = addr.clone();
    thread::spawn(move || {
        let _ = run_multi_client_echo_server(&server_addr);
    });

    assert!(wait_for_server(&addr, 20), "Server failed to start");

    for i in 0..3 {
        let mut stream = TcpStream::connect(&addr).unwrap();
        let msg = format!("Sequential{}", i);

        writeln!(stream, "{}", msg).unwrap();
        stream.flush().unwrap();

        let mut reader = BufReader::new(&stream);
        let mut response = String::new();
        reader.read_line(&mut response).unwrap();

        assert_eq!(response.trim(), msg);
        drop(stream);
    }
}

// =============================================================================
// Mission 3: Chat Server with Broadcast
// =============================================================================

#[test]
fn mission_3_shared_clients_creation() {
    let clients = new_shared_clients();
    assert_eq!(clients.lock().unwrap().len(), 0);
}

#[test]
fn mission_3_broadcast_to_multiple_clients() {
    let port = get_test_port();
    let addr = format!("127.0.0.1:{}", port);

    let server_addr = addr.clone();
    thread::spawn(move || {
        let _ = run_chat_server(&server_addr);
    });

    assert!(wait_for_server(&addr, 20), "Server failed to start");

    let mut client1 = TcpStream::connect(&addr).unwrap();
    client1
        .set_read_timeout(Some(Duration::from_millis(500)))
        .unwrap();

    let mut client2 = TcpStream::connect(&addr).unwrap();
    client2
        .set_read_timeout(Some(Duration::from_millis(500)))
        .unwrap();

    thread::sleep(Duration::from_millis(100));

    writeln!(client1, "Hello from client1").unwrap();
    client1.flush().unwrap();

    thread::sleep(Duration::from_millis(100));

    let mut reader2 = BufReader::new(&client2);
    let mut received = String::new();
    let result = reader2.read_line(&mut received);

    assert!(result.is_ok(), "Client2 should receive the broadcast");
    assert!(
        received.contains("Hello from client1"),
        "Message content mismatch: {}",
        received
    );
}

#[test]
fn mission_3_sender_receives_own_message() {
    let port = get_test_port();
    let addr = format!("127.0.0.1:{}", port);

    let server_addr = addr.clone();
    thread::spawn(move || {
        let _ = run_chat_server(&server_addr);
    });

    assert!(wait_for_server(&addr, 20), "Server failed to start");

    let stream = TcpStream::connect(&addr).unwrap();
    stream
        .set_read_timeout(Some(Duration::from_millis(500)))
        .unwrap();

    let mut writer = stream.try_clone().unwrap();
    let mut reader = BufReader::new(stream);

    thread::sleep(Duration::from_millis(50));

    writeln!(writer, "Test message").unwrap();
    writer.flush().unwrap();

    thread::sleep(Duration::from_millis(100));

    let mut received = String::new();
    let result = reader.read_line(&mut received);

    assert!(result.is_ok(), "Sender should receive own message (echo)");
    assert!(
        received.contains("Test message"),
        "Message mismatch: {}",
        received
    );
}

#[test]
fn mission_3_multiple_messages_broadcast() {
    let port = get_test_port();
    let addr = format!("127.0.0.1:{}", port);

    let server_addr = addr.clone();
    thread::spawn(move || {
        let _ = run_chat_server(&server_addr);
    });

    assert!(wait_for_server(&addr, 20), "Server failed to start");

    let client1 = TcpStream::connect(&addr).unwrap();
    client1
        .set_read_timeout(Some(Duration::from_millis(500)))
        .unwrap();

    let client2 = TcpStream::connect(&addr).unwrap();
    client2
        .set_read_timeout(Some(Duration::from_millis(500)))
        .unwrap();

    thread::sleep(Duration::from_millis(100));

    let mut writer1 = client1.try_clone().unwrap();
    let mut writer2 = client2.try_clone().unwrap();
    let mut reader1 = BufReader::new(client1);
    let mut reader2 = BufReader::new(client2);

    writeln!(writer1, "Msg1").unwrap();
    writer1.flush().unwrap();
    thread::sleep(Duration::from_millis(50));

    writeln!(writer2, "Msg2").unwrap();
    writer2.flush().unwrap();
    thread::sleep(Duration::from_millis(50));

    let mut msg1_received_by_1 = String::new();
    let mut msg1_received_by_2 = String::new();

    let _ = reader1.read_line(&mut msg1_received_by_1);
    let _ = reader2.read_line(&mut msg1_received_by_2);

    assert!(
        msg1_received_by_1.contains("Msg1") || msg1_received_by_2.contains("Msg1"),
        "Msg1 should be broadcast"
    );
}
