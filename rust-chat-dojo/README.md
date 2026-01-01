# Rust Chat Dojo

채팅 서버를 만들면서 Rust 스마트 포인터를 **통합 연습**하는 수련장입니다.

## 빠른 시작

```bash
cd rust-chat-dojo

cargo test           # 전체 테스트 (처음엔 모두 실패)
cargo test mission_1 # User, Room
cargo test mission_2 # SingleThreadChatServer
cargo test mission_3 # MultiThreadChatServer
cargo test mission_4 # MessageBroker
cargo test mission_5 # StatsCollector
```

## 학습 경로

| 임무 | 주제 | 핵심 개념 |
|------|------|----------|
| 1 | User와 Room 기본 구조 | `Rc<RefCell<T>>`, `Weak` |
| 2 | 싱글 스레드 ChatServer | `Rc<RefCell<T>>` 통합 |
| 3 | 멀티 스레드 ChatServer | `Arc<RwLock<T>>`, `Arc<Mutex<T>>` |
| 4 | 메시지 브로커 | `mpsc::channel` |
| 5 | 통계 수집기 | `Arc<RwLock<T>>` |

## 상세 안내

- 학습 계획: [docs/README.md](docs/README.md)
- 도움 요청: [docs/Assistance.md](docs/Assistance.md)
