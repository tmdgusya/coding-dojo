# Rust Smart Pointer Dojo

Rust 스마트 포인터를 **반복 연습**으로 익히는 수련장입니다.

## 빠른 시작

```bash
cd rust-smartptr-dojo

cargo test           # 전체 테스트 (처음엔 모두 실패)
cargo test mission_1 # FsNode (트리 구조)
cargo test mission_2 # EventBus (옵저버)
cargo test mission_3 # LruCache
cargo test mission_4 # ConnectionPool
cargo test mission_5 # ConfigManager
cargo test mission_6 # ResultCollector
```

## 학습 경로

| 임무 | 시나리오 | 핵심 개념 |
|------|----------|----------|
| 1 | 파일 시스템 트리 | `Rc<RefCell<T>>` + `Weak` |
| 2 | 이벤트 버스 | `Rc<RefCell<Vec<...>>>` |
| 3 | LRU 캐시 | `Rc` + `RefCell` + `HashMap` |
| 4 | 연결 풀 | `Arc<Mutex<Vec<...>>>` |
| 5 | 공유 설정 | `Arc<RwLock<T>>` |
| 6 | 병렬 작업 수집기 | `Arc<Mutex<T>>` + channel |

## 상세 안내

- 학습 계획: [docs/README.md](docs/README.md)
- 도움 요청: [docs/Assistance.md](docs/Assistance.md)
