# Rust Worker Dojo

Rust 동시성(Concurrency)을 배우기 위한 수련장입니다.

## 빠른 시작

```bash
cd rust-worker-dojo

cargo test           # 전체 테스트 (처음엔 모두 실패)
cargo test mission_1 # Box
cargo test mission_2 # Rc
cargo test mission_3 # RefCell
cargo test mission_4 # Thread
cargo test mission_5 # Channel
cargo test mission_6 # Arc + Mutex
cargo test mission_7 # Worker Pool
```

## 학습 경로

| 임무 | 주제 | 핵심 개념 |
|------|------|----------|
| 1 | 스마트 포인터 기초 | `Box<T>` |
| 2 | 참조 카운팅 | `Rc<T>` |
| 3 | 내부 가변성 | `RefCell<T>` |
| 4 | 스레드 기초 | `thread::spawn`, `join` |
| 5 | 메시지 패싱 | `mpsc::channel` |
| 6 | 공유 상태 | `Arc<Mutex<T>>` |
| 7 | **최종** | Worker Pool |

## 상세 안내

- 학습 계획: [docs/README.md](docs/README.md)
- 도움 요청: [docs/Assistance.md](docs/Assistance.md)
