# Rust TCP Chat Dojo

실제 TCP 소켓으로 채팅 서버를 구현하는 수련장입니다.

## 빠른 시작

```bash
cd rust-tcp-chat-dojo

cargo test           # 전체 테스트 (처음엔 모두 실패)
cargo test mission_1 # Echo Server
cargo test mission_2 # Multi-Client Echo
cargo test mission_3 # Chat Broadcast
```

## 학습 경로

| 임무 | 주제 | 핵심 개념 |
|------|------|----------|
| 1 | Echo Server | `TcpListener`, `TcpStream`, `BufReader` |
| 2 | Multi-Client Echo | `thread::spawn`, `move` closure |
| 3 | Chat with Broadcast | `Arc<Mutex<Vec<TcpStream>>>`, `try_clone()` |

## 직접 실행

```bash
# Echo Server
cargo run --bin echo_server

# Chat Server
cargo run --bin chat_server

# Chat Client
cargo run --bin chat_client
```

## 상세 안내

- 학습 계획: [docs/README.md](docs/README.md)
- 도움 요청: [docs/Assistance.md](docs/Assistance.md)
