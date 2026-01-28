# Rust Smart Pointer Dojo - 패턴 반복 훈련

**Rc, RefCell, Arc, Mutex, RwLock**을 다양한 시나리오에서 반복 연습합니다.

같은 도구를 여러 상황에서 사용하면서 손에 익히는 것이 목표입니다.

## 학습 경로

| 임무 | 시나리오 | 핵심 개념 |
|------|----------|----------|
| 1 | 파일 시스템 트리 | `Rc<RefCell<T>>` + `Weak` |
| 2 | 이벤트 버스 | `Rc<RefCell<Vec<...>>>` |
| 3 | LRU 캐시 | `Rc` + `RefCell` + `HashMap` |
| 4 | 연결 풀 | `Arc<Mutex<Vec<...>>>` |
| 5 | 공유 설정 | `Arc<RwLock<T>>` |
| 6 | 병렬 작업 수집기 | `Arc<Mutex<T>>` + channel |

## 빠른 시작

```bash
cd rust-smartptr-dojo

# 전체 테스트 (처음엔 모두 실패)
cargo test

# 임무별 테스트
cargo test mission_1    # FsNode (트리 구조)
cargo test mission_2    # EventBus (옵저버)
cargo test mission_3    # LruCache
cargo test mission_4    # ConnectionPool
cargo test mission_5    # ConfigManager
cargo test mission_6    # ResultCollector
```

---

## 임무 상세

### 임무 1: 파일 시스템 트리

**목표**: 부모-자식 관계가 있는 트리 구조 구현

```
root/
├── docs/
│   └── readme.txt
└── src/
    └── main.rs
```

**핵심 패턴**:
- 부모 → 자식: `Rc<FsNode>` (소유)
- 자식 → 부모: `Weak<FsNode>` (순환 참조 방지)

**구현할 것**:
- `FsNode::new_file()`, `new_dir()`
- `FsNode::add_child()` - 부모-자식 연결
- `FsNode::parent_name()` - Weak 참조로 부모 접근
- `FsNode::full_path()` - 루트까지 경로 추적

---

### 임무 2: 이벤트 버스 (옵저버 패턴)

**목표**: 이벤트 발행-구독 시스템 구현

```
EventBus
  ├── Logger("app")
  └── Logger("audit")
  
bus.publish("user_login")
  → app:   [app] user_login
  → audit: [audit] user_login
```

**핵심 패턴**:
- 리스너 저장: `Vec<Rc<RefCell<dyn EventListener>>>`
- 트레이트 객체로 다양한 리스너 타입 지원

**구현할 것**:
- `EventBus::subscribe()` - 리스너 등록
- `EventBus::publish()` - 모든 리스너에게 전달

---

### 임무 3: LRU 캐시

**목표**: Least Recently Used 캐시 구현

```
capacity = 2

put("a", 1)  → [a]
put("b", 2)  → [b, a]
get("a")     → [a, b]  (a가 최근 사용됨)
put("c", 3)  → [c, a]  (b 제거됨)
```

**핵심 패턴**:
- `HashMap<String, Rc<CacheNode<V>>>` - O(1) 조회
- 이중 연결 리스트로 순서 관리
- `RefCell`로 노드의 prev/next 수정

**구현할 것**:
- `LruCache::get()` - 조회 + 최근으로 이동
- `LruCache::put()` - 저장 + LRU 제거

---

### 임무 4: 연결 풀

**목표**: 스레드 안전한 연결 풀 구현

```
pool (size=2)
  ├── Connection 0: available
  └── Connection 1: available

Thread A: acquire() → Connection 0 (in_use)
Thread B: acquire() → Connection 1 (in_use)
Thread C: acquire() → None (모두 사용 중)
Thread A: release(0) → Connection 0 (available)
Thread C: acquire() → Connection 0 (in_use)
```

**핵심 패턴**:
- `Arc<Mutex<Vec<Connection>>>` - 스레드 안전한 공유
- `clone_pool()`로 다른 스레드에 전달

**구현할 것**:
- `ConnectionPool::acquire()` - 연결 획득
- `ConnectionPool::release()` - 연결 반납
- `ConnectionPool::execute()` - 획득 → 실행 → 반납

---

### 임무 5: 공유 설정

**목표**: 읽기 많고 쓰기 적은 설정 공유

```
ConfigManager
  └── Arc<RwLock<Config>>
  
Thread A: get_config()  ─┐
Thread B: get_config()  ─┼─ 동시 읽기 OK
Thread C: get_config()  ─┘

Thread D: update(...)   ─── 단독 쓰기 (다른 읽기 차단)
```

**핵심 패턴**:
- `Arc<RwLock<T>>` - 여러 읽기 또는 하나의 쓰기
- `Mutex`보다 읽기 성능 우수

**구현할 것**:
- `ConfigManager::get_config()` - 읽기 락
- `ConfigManager::update()` - 쓰기 락

---

### 임무 6: 병렬 작업 수집기

**목표**: 여러 스레드의 결과를 안전하게 수집

```
[1, 2, 3, 4, 5]
    │
    ├─ Thread 1: 1² = 1
    ├─ Thread 2: 2² = 4
    ├─ Thread 3: 3² = 9
    ├─ Thread 4: 4² = 16
    └─ Thread 5: 5² = 25
    │
    ▼
[1, 4, 9, 16, 25]
```

**핵심 패턴**:
- `Arc<Mutex<Vec<T>>>` - 결과 수집
- `mpsc::channel` - 작업 결과 전송

**구현할 것**:
- `ResultCollector::add()` - 스레드 안전한 추가
- `parallel_square()` - 병렬 제곱 계산
- `parallel_with_channel()` - 채널로 결과 수집

---

## 핵심 개념 정리

### 단일 스레드 vs 멀티 스레드

| 상황 | 사용 | 이유 |
|------|------|------|
| 단일 스레드, 여러 소유자 | `Rc<T>` | Reference Counting |
| 단일 스레드, 내부 가변성 | `RefCell<T>` | 런타임 빌림 검사 |
| 멀티 스레드, 여러 소유자 | `Arc<T>` | Atomic Reference Counting |
| 멀티 스레드, 내부 가변성 | `Mutex<T>` | 상호 배제 락 |
| 멀티 스레드, 읽기 많음 | `RwLock<T>` | 읽기-쓰기 락 |

### 조합 패턴

| 패턴 | 용도 |
|------|------|
| `Rc<RefCell<T>>` | 단일 스레드에서 공유 + 수정 |
| `Arc<Mutex<T>>` | 멀티 스레드에서 공유 + 수정 |
| `Arc<RwLock<T>>` | 멀티 스레드에서 읽기 많은 공유 |
| `Weak<T>` | 순환 참조 방지 (부모 참조) |

---

## 도움이 필요하면

[docs/Assistance.md](./Assistance.md) 참고
