// =============================================================================
// Rust TCP Chat Dojo
//
// 실제 TCP 소켓 통신으로 채팅 서버를 구현합니다.
// std::net만 사용하여 핵심 개념을 학습합니다.
//
// 학습 목표:
// - TcpListener, TcpStream 사용법
// - Read, Write trait
// - 멀티스레드 클라이언트 처리
// - 메시지 브로드캐스팅
// =============================================================================

use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

// =============================================================================
// 임무 1: Echo Server
//
// 가장 기본적인 TCP 서버입니다.
// 클라이언트가 보낸 메시지를 그대로 돌려보냅니다.
// =============================================================================

/// 단일 클라이언트를 처리하는 Echo 핸들러
///
/// 클라이언트로부터 한 줄을 읽고, 그대로 돌려보냅니다.
/// 빈 줄이 오면 연결을 종료합니다.
///
/// # 구현 힌트
/// - BufReader로 스트림을 감싸면 read_line() 사용 가능
/// - write_all()과 flush()로 응답 전송
pub fn handle_echo_client(stream: TcpStream) -> io::Result<()> {
    let reader_stream = stream.try_clone();
    let writer_stream = stream.try_clone();

    let reader = BufReader::new(reader_stream.unwrap());
    let mut writer = std::io::BufWriter::new(writer_stream.unwrap());

    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            break;
        }
        writer.write_all(line.as_bytes())?;
        writer.write_all(b"\n")?;
        writer.flush()?;
    }

    Ok(())
}

/// Echo 서버를 시작합니다.
///
/// 지정된 주소에서 연결을 기다리고, 각 클라이언트를 처리합니다.
/// 단일 스레드로 동작합니다 (한 번에 하나의 클라이언트만).
///
/// # 구현 힌트
/// - TcpListener::bind()로 소켓 바인딩
/// - listener.incoming()으로 연결 수락
pub fn run_echo_server(addr: &str) -> io::Result<()> {
    let listener = TcpListener::bind(addr);
    listener.unwrap().incoming().for_each(|stream| {
        let stream = stream.unwrap();
        std::thread::spawn(move || {
            handle_echo_client(stream).unwrap();
        });
    });

    Ok(())
}

// =============================================================================
// 임무 2: Multi-Client Echo Server
//
// 여러 클라이언트를 동시에 처리할 수 있는 서버입니다.
// 각 클라이언트를 별도 스레드에서 처리합니다.
// =============================================================================

/// 멀티 클라이언트 Echo 서버
///
/// 각 연결을 새 스레드에서 처리하여 동시에 여러 클라이언트를 지원합니다.
///
/// # 구현 힌트
/// - thread::spawn()으로 새 스레드 생성
/// - move 클로저로 stream 소유권 이전
pub fn run_multi_client_echo_server(addr: &str) -> io::Result<()> {
    let listener = TcpListener::bind(addr)?;
    for stream in listener.incoming() {
        let stream = stream?;
        std::thread::spawn(move || {
            handle_echo_client(stream).unwrap();
        });
    }

    Ok(())
}

// =============================================================================
// 임무 3: Chat Server with Broadcast
//
// 실제 채팅 서버입니다.
// 한 클라이언트가 보낸 메시지를 모든 클라이언트에게 전달합니다.
// =============================================================================

/// 연결된 클라이언트 목록을 공유하기 위한 타입
pub type SharedClients = Arc<Mutex<Vec<TcpStream>>>;

/// 새 클라이언트 목록을 생성합니다.
pub fn new_shared_clients() -> SharedClients {
    Arc::new(Mutex::new(Vec::new()))
}

/// 모든 클라이언트에게 메시지를 브로드캐스트합니다.
///
/// sender_addr: 메시지를 보낸 클라이언트 주소 (자기 자신에게는 안 보냄)
///
/// # 구현 힌트
/// - 각 클라이언트의 try_clone()으로 스트림 복제
/// - 실패한 클라이언트는 건너뛰기 (연결이 끊어졌을 수 있음)
pub fn broadcast(clients: &SharedClients, message: &str, sender_addr: Option<&str>) {
    clients.lock().unwrap().iter().for_each(|client| {
        if let Err(e) = client
            .try_clone()
            .and_then(|mut cloned| cloned.write_all(message.as_bytes()))
        {
            eprintln!("Failed to broadcast message: {}", e);
        }
    });
}

/// 채팅 클라이언트 핸들러
///
/// 클라이언트로부터 메시지를 받아 모든 클라이언트에게 브로드캐스트합니다.
///
/// # 구현 힌트
/// - 클라이언트 주소를 포함한 메시지 형식: "[addr] message"
/// - 연결 종료 시 clients에서 제거
pub fn handle_chat_client(stream: TcpStream, clients: SharedClients) -> io::Result<()> {
    stream.try_clone().and_then(|mut cloned| {
        let addr = cloned.peer_addr()?;
        let sender_addr = addr.to_string();
        let mut buffer = vec![0; 1024];
        loop {
            match cloned.read(&mut buffer) {
                Ok(0) => {
                    eprintln!("Client disconnected");
                    clients
                        .lock()
                        .unwrap()
                        .retain(|client| client.peer_addr().map(|a| a != addr).unwrap_or(false));
                    break;
                }
                Ok(n) => {
                    let message = format!(
                        "[{}] {}\n",
                        sender_addr,
                        String::from_utf8_lossy(&buffer[..n]).trim()
                    );
                    broadcast(&clients, &message, Some(&sender_addr));
                }
                Err(e) => {
                    eprintln!("Failed to read from client: {}", e);
                    break;
                }
            }
        }
        Ok(())
    })
}

/// 채팅 서버 실행
///
/// # 구현 힌트
/// - 새 연결 시 clients에 추가
/// - 각 클라이언트를 별도 스레드에서 처리
pub fn run_chat_server(addr: &str) -> io::Result<()> {
    let clients: SharedClients = Arc::new(Mutex::new(Vec::new()));
    let listener = TcpListener::bind(addr)?;

    for stream in listener.incoming() {
        let clients = clients.clone();
        let stream = stream?;
        clients.lock().unwrap().push(stream.try_clone()?);
        thread::spawn(move || {
            if let Err(e) = handle_chat_client(stream, clients) {
                eprintln!("Failed to handle client: {}", e);
            }
        });
    }

    Ok(())
}

// =============================================================================
// 유틸리티 함수
// =============================================================================

/// 테스트용: 서버에 연결하고 메시지를 보낸 후 응답을 받습니다.
pub fn send_and_receive(addr: &str, message: &str) -> io::Result<String> {
    let mut stream = TcpStream::connect(addr)?;
    stream.write_all(message.as_bytes())?;
    stream.write_all(b"\n")?;
    stream.flush()?;

    let mut reader = BufReader::new(&stream);
    let mut response = String::new();
    reader.read_line(&mut response)?;
    Ok(response)
}

/// 테스트용: 서버에 연결하고 여러 메시지를 주고받습니다.
pub fn connect_and_chat(addr: &str) -> io::Result<TcpStream> {
    TcpStream::connect(addr)
}

// =============================================================================
// 테스트 헬퍼
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shared_clients_creation() {
        let clients = new_shared_clients();
        assert_eq!(clients.lock().unwrap().len(), 0);
    }
}
