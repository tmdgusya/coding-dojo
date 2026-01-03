# Rust TCP Chat Dojo - 학습 경로

실제 TCP 소켓으로 채팅 서버를 구현하며 네트워크 프로그래밍의 기초를 익힙니다.

## 핵심 개념

```
클라이언트                    서버
    │                          │
    ├── connect() ───────────→ │ accept()
    │                          │
    ├── write() ─────────────→ │ read()
    │                          │
    │ ←───────────── write() ──┤
    │                          │
    └── close() ─────────────→ │
```

## 학습 경로

| 임무 | 주제 | 핵심 개념 |
|------|------|----------|
| 1 | Echo Server | `TcpListener`, `TcpStream`, `BufReader` |
| 2 | Multi-Client Echo | `thread::spawn`, `move` closure |
| 3 | Chat with Broadcast | `Arc<Mutex<Vec<TcpStream>>>`, `try_clone()` |

## 빠른 시작

```bash
cd rust-tcp-chat-dojo

# 전체 테스트 (처음엔 모두 실패)
cargo test

# 임무별 테스트
cargo test mission_1   # Echo Server
cargo test mission_2   # Multi-Client
cargo test mission_3   # Chat Broadcast
```

---

## 임무 1: Echo Server

**목표**: TcpListener/TcpStream 기본 사용법

```
클라이언트: "Hello"  ──→  서버
클라이언트  ←──  서버: "Hello"
```

**구현할 것**:
- `handle_echo_client()`: 클라이언트 메시지를 읽고 그대로 반환
- `run_echo_server()`: 지정 주소에서 연결 대기 및 처리

**필수 API**:
```rust
TcpListener::bind(addr)?         // 소켓 바인딩
listener.incoming()              // 연결 수락 이터레이터
BufReader::new(stream)           // 버퍼링된 읽기
reader.read_line(&mut buf)?      // 한 줄 읽기
stream.write_all(bytes)?         // 바이트 전송
stream.flush()?                  // 버퍼 플러시
```

**테스트**:
```bash
cargo test mission_1
```

---

## 임무 2: Multi-Client Echo Server

**목표**: 여러 클라이언트를 동시에 처리

```
클라이언트1 ──┐
             ├──→ 서버 (각각 별도 스레드)
클라이언트2 ──┘
```

**구현할 것**:
- `run_multi_client_echo_server()`: 각 연결을 새 스레드에서 처리

**핵심 패턴**:
```rust
for stream in listener.incoming() {
    let stream = stream?;
    thread::spawn(move || {
        handle_echo_client(stream)
    });
}
```

**주의점**:
- `move` 클로저로 stream 소유권 이전
- 스레드 내 에러 처리

**테스트**:
```bash
cargo test mission_2
```

---

## 임무 3: Chat Server with Broadcast

**목표**: 한 클라이언트의 메시지를 모든 클라이언트에게 전달

```
클라이언트A: "Hi" ──→ 서버 ──→ 클라이언트B, C, D
```

**구현할 것**:
- `broadcast()`: 모든 클라이언트에게 메시지 전송
- `handle_chat_client()`: 메시지 수신 및 브로드캐스트
- `run_chat_server()`: 클라이언트 목록 관리

**핵심 구조**:
```rust
type SharedClients = Arc<Mutex<Vec<TcpStream>>>;
```

**어려운 점**:
- 읽기용 스트림과 쓰기용 스트림 분리 (`try_clone()`)
- 락 데드락 방지
- 연결 끊긴 클라이언트 처리

**테스트**:
```bash
cargo test mission_3
```

---

## 직접 테스트하기

### Echo Server

터미널 1:
```bash
cargo run --bin echo_server
```

터미널 2:
```bash
nc localhost 7878
Hello
# → Hello (에코 응답)
```

### Chat Server

터미널 1:
```bash
cargo run --bin chat_server
```

터미널 2, 3:
```bash
cargo run --bin chat_client
# 메시지를 입력하면 다른 클라이언트에게 전달
```

---

## 핵심 패턴 정리

### Read vs Write

| Read 측 | Write 측 |
|---------|----------|
| `BufReader::new(stream)` | `stream` 직접 사용 |
| `reader.read_line()` | `stream.write_all()` |
| 블로킹 (데이터 올 때까지 대기) | `flush()` 필수 |

### 소유권과 스트림

```rust
// 읽기/쓰기 분리가 필요할 때
let read_stream = stream.try_clone()?;  // 복제
let write_stream = stream;              // 원본

// 스레드로 넘길 때
thread::spawn(move || {
    // stream 소유권 이동
});
```

### Arc<Mutex<T>> 패턴

```rust
let clients: SharedClients = Arc::new(Mutex::new(Vec::new()));

// 클론해서 스레드로
let clients_clone = Arc::clone(&clients);
thread::spawn(move || {
    let mut locked = clients_clone.lock().unwrap();
    locked.push(stream);
});
```

---

## 도움이 필요하면

[Assistance.md](./Assistance.md) 참고
