# Rust Worker Dojo - 동시성 마스터하기

Rust의 **"Fearless Concurrency"**를 체험하는 수련장입니다.

소유권 시스템 덕분에 Rust는 **컴파일 타임에 데이터 레이스를 방지**합니다.
이번 과제에서는 스마트 포인터와 동시성 기초를 배우고, 최종적으로 **Worker Pool**을 구현합니다.

## 학습 경로

| 임무 | 주제 | 핵심 개념 |
|------|------|----------|
| 1 | 스마트 포인터 기초 | `Box<T>` - 힙 할당 |
| 2 | 참조 카운팅 | `Rc<T>` - 여러 소유자 |
| 3 | 내부 가변성 | `RefCell<T>` - 런타임 빌림 검사 |
| 4 | 스레드 기초 | `thread::spawn`, `join`, `move` |
| 5 | 메시지 패싱 | `mpsc::channel` - 스레드 간 통신 |
| 6 | 공유 상태 | `Arc<Mutex<T>>` - 스레드 안전한 공유 |
| 6.5 | Graceful Shutdown | `recv() Err`, `Option::take()`, `Drop` |
| 7 | **최종** | Worker Pool 구현 |

## 선수 지식

rust-dojo (minigrep)에서 배운 것:
- 소유권과 빌림
- Result, Option
- 구조체, 트레이트
- 생명주기
- 이터레이터

## 빠른 시작

```bash
cd rust-worker-dojo

# 전체 테스트 (처음엔 모두 실패)
cargo test

# 임무별 테스트
cargo test mission_1    # Box
cargo test mission_2    # Rc
cargo test mission_3    # RefCell
cargo test mission_4    # Thread
cargo test mission_5    # Channel
cargo test mission_6    # Arc + Mutex
cargo test mission_6_5  # Graceful Shutdown
cargo test mission_7    # Worker Pool
```

## 임무 상세

### 임무 1: Box<T> - 힙에 데이터 저장하기

**목표**: 재귀적 데이터 구조 만들기

```rust
// 이건 컴파일 안 됨 - 왜?
enum List {
    Cons(i32, List),
    Nil,
}

// Box를 쓰면?
enum List {
    Cons(i32, Box<List>),
    Nil,
}
```

**구현할 것**:
- `List` enum과 기본 연산
- 리스트 길이, 합계 계산

---

### 임무 2: Rc<T> - 여러 소유자

**목표**: 하나의 데이터를 여러 곳에서 소유하기

```
    a (3) -> b (2) -> c (1) -> Nil
              ^
    d (4) ---┘
```

두 리스트가 일부를 공유하는 구조

**구현할 것**:
- `Rc`를 사용한 공유 리스트
- 참조 카운트 확인

---

### 임무 3: RefCell<T> - 내부 가변성

**목표**: 불변 참조를 통해 내부 값 변경하기

```rust
// 이게 가능할까?
let data = Rc::new(vec![1, 2, 3]);
data.push(4);  // 에러! Rc는 불변

// RefCell을 쓰면?
let data = Rc::new(RefCell::new(vec![1, 2, 3]));
data.borrow_mut().push(4);  // OK!
```

**구현할 것**:
- 캐시 시스템 (memoization)
- Mock 객체 패턴

---

### 임무 4: Thread - 스레드 기초

**목표**: 여러 작업을 병렬로 실행하기

```rust
let handle = thread::spawn(|| {
    // 다른 스레드에서 실행
});
handle.join().unwrap();  // 완료 대기
```

**구현할 것**:
- 스레드 생성과 조인
- `move` 클로저로 소유권 이동

---

### 임무 5: Channel - 메시지 패싱

**목표**: 스레드 간 안전한 통신

```rust
let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    tx.send("hello").unwrap();
});

let msg = rx.recv().unwrap();  // "hello"
```

**구현할 것**:
- 생산자-소비자 패턴
- 여러 생산자 (tx.clone())

---

### 임무 6: Arc<Mutex<T>> - 공유 상태

**목표**: 여러 스레드에서 안전하게 데이터 공유

```rust
let counter = Arc::new(Mutex::new(0));

for _ in 0..10 {
    let counter = Arc::clone(&counter);
    thread::spawn(move || {
        let mut num = counter.lock().unwrap();
        *num += 1;
    });
}
```

**구현할 것**:
- 스레드 안전한 카운터
- 공유 데이터 구조

---

### 임무 6.5: Graceful Shutdown 패턴

**목표**: Worker Pool 구현 전에 핵심 패턴 익히기

```text
[Main Thread]              [Worker Thread]
     |                           |
     |-- execute(job1) --------> |  recv() -> Ok(job1) -> 실행
     |-- execute(job2) --------> |  recv() -> Ok(job2) -> 실행
     |                           |
  (drop)                         |
     |-- sender drop ----------> |  recv() -> Err! -> 루프 탈출
     |-- join() --------------> |  스레드 종료
     |                           X
```

**배울 것**:
1. `recv()`가 `Err`을 반환하는 조건: 모든 Sender가 drop되었을 때
2. `Option<T>::take()`: 소유권을 꺼내면서 None으로 교체
3. `Drop` 트레이트에서 정리 작업 순서

**구현할 것**:
- `JobRunner` 구조체 (단일 워커)
- `execute()` 메서드
- `Drop` 트레이트로 graceful shutdown

---

### 임무 7: Worker Pool (최종)

**목표**: 모든 개념을 통합한 Worker Pool 구현

```rust
let pool = ThreadPool::new(4);  // 4개 워커

for job in jobs {
    pool.execute(|| {
        // 작업 수행
    });
}
```

**구현할 것**:
- `ThreadPool` 구조체
- 작업 큐와 워커 스레드
- Graceful shutdown

---

## 왜 이게 중요한가?

| 언어 | 동시성 안전 보장 |
|------|-----------------|
| C/C++ | 런타임 (데이터 레이스 가능) |
| Java | 런타임 (synchronized, volatile) |
| Go | 런타임 (race detector) |
| **Rust** | **컴파일 타임!** |

Rust에서 데이터 레이스가 있는 코드는 **컴파일 자체가 안 됩니다**.

## 도움이 필요하면

[docs/Assistance.md](./Assistance.md) 참고
