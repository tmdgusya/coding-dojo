# Rust Chat Dojo - 채팅 서버 시뮬레이터

**하나의 프로젝트**에서 모든 스마트 포인터 개념을 통합 연습합니다.

간단한 채팅 서버를 구현하면서 Rc, RefCell, Arc, Mutex, RwLock, Weak, Channel을 맥락 있게 배웁니다.

## 프로젝트 구조

```
ChatServer
├── User (사용자)
│   ├── 여러 Room에 참여 가능
│   └── 메시지 수신함 (inbox)
├── Room (채팅방)
│   ├── 여러 User가 참여
│   └── 메시지 히스토리
└── Message (메시지)
    ├── sender_id
    ├── content
    └── timestamp
```

## 학습 경로

| 임무 | 주제 | 핵심 개념 |
|------|------|----------|
| 1 | User와 Room 기본 구조 | `Rc<RefCell<T>>`, `Weak` |
| 2 | 싱글 스레드 ChatServer | `Rc<RefCell<T>>` 통합 |
| 3 | 멀티 스레드 ChatServer | `Arc<RwLock<T>>`, `Arc<Mutex<T>>` |
| 4 | 메시지 브로커 | `mpsc::channel`, 워커 스레드 |
| 5 | 통계 수집기 | `Arc<RwLock<T>>` |

## 빠른 시작

```bash
cd rust-chat-dojo

# 전체 테스트 (처음엔 모두 실패)
cargo test

# 임무별 테스트
cargo test mission_1    # User, Room
cargo test mission_2    # SingleThreadChatServer
cargo test mission_3    # MultiThreadChatServer
cargo test mission_4    # MessageBroker
cargo test mission_5    # StatsCollector
```

---

## 임무 상세

### 임무 1: User와 Room 기본 구조

**목표**: Rc<RefCell<T>>와 Weak로 양방향 관계 구현

```
User ←--Weak--- Room
  │               │
  └---Weak------→ │
```

- User는 여러 Room에 참여 (Weak 참조)
- Room은 여러 User를 보유 (Weak 참조)
- 순환 참조 방지를 위해 양쪽 모두 Weak 사용

**구현할 것**:
- `User::new()`, `Room::new()`
- `User::join_room()`, `Room::add_member()`
- `Room::broadcast()` - 모든 멤버에게 메시지 전달

---

### 임무 2: 싱글 스레드 ChatServer

**목표**: HashMap으로 사용자/방 관리

```rust
SingleThreadChatServer
├── users: HashMap<UserId, Rc<RefCell<User>>>
├── rooms: HashMap<RoomId, Rc<RefCell<Room>>>
└── next_*_id: UserId, RoomId
```

**구현할 것**:
- `create_user()`, `create_room()`
- `join_room()`, `leave_room()`
- `send_message()` - 방의 모든 멤버에게 전달
- 에러 처리 (존재하지 않는 user/room)

---

### 임무 3: 멀티 스레드 ChatServer

**목표**: Arc, RwLock, Mutex로 스레드 안전하게 전환

```rust
MultiThreadChatServer
├── users: Arc<RwLock<HashMap<UserId, Arc<ThreadSafeUser>>>>
├── rooms: Arc<RwLock<HashMap<RoomId, Arc<ThreadSafeRoom>>>>
└── next_*_id: Arc<Mutex<UserId>>

ThreadSafeUser
├── id, name
└── inbox: Arc<Mutex<Vec<Message>>>

ThreadSafeRoom
├── id, name
├── members: RwLock<Vec<UserId>>
└── history: Mutex<Vec<Message>>
```

**왜 이렇게?**:
- `users/rooms`: 읽기가 많으므로 RwLock
- `next_*_id`: 증가만 하므로 Mutex
- `inbox`: 추가만 하므로 Mutex
- `members`: 읽기(조회)가 쓰기(join/leave)보다 많음 → RwLock
- `history`: 추가만 하므로 Mutex

**구현할 것**:
- `ThreadSafeUser`, `ThreadSafeRoom`
- `MultiThreadChatServer` 전체

---

### 임무 4: 메시지 브로커

**목표**: Channel로 비동기 메시지 처리

```
Main Thread          Worker Thread
     │                    │
     ├── send_command ──→ │
     │                    ├── process
     ├── send_command ──→ │
     │                    ├── process
     ├── Shutdown ──────→ │
     │                    └── exit
```

**구현할 것**:
- `MessageBroker::new()` - 워커 스레드 시작
- `send_command()` - 채널로 커맨드 전송
- `shutdown()` - Graceful shutdown

---

### 임무 5: 통계 수집기

**목표**: 실시간 서버 통계 수집

```rust
ServerStats {
    total_messages: u64,
    total_joins: u64,
    total_leaves: u64,
    active_users: u64,
    active_rooms: u64,
}
```

**구현할 것**:
- `StatsCollector::record_*()` - 통계 기록
- `get_stats()` - 스냅샷 조회
- 여러 스레드에서 동시 기록 지원

---

## 핵심 패턴 비교

### 싱글 스레드 vs 멀티 스레드

| 싱글 스레드 | 멀티 스레드 |
|-------------|-------------|
| `Rc<RefCell<T>>` | `Arc<Mutex<T>>` |
| `Rc::clone()` | `Arc::clone()` |
| `.borrow()` | `.lock().unwrap()` |
| `.borrow_mut()` | `.lock().unwrap()` |

### RwLock vs Mutex

| RwLock | Mutex |
|--------|-------|
| 여러 읽기 OR 하나의 쓰기 | 하나만 접근 |
| 읽기 많을 때 유리 | 쓰기 많을 때 |
| `read()`, `write()` | `lock()` |

### Weak 참조 사용 시점

```
강한 참조 사이클 방지:
Parent ──(Rc)──→ Child
   ↑                │
   └────(Weak)──────┘
```

---

## 도움이 필요하면

[docs/Assistance.md](./Assistance.md) 참고
