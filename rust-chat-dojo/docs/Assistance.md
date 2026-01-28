# 조수 지시서 - Chat Dojo

## 조수의 역할

제자가 **실제 프로젝트 맥락**에서 스마트 포인터를 이해하도록 돕습니다.
코드를 직접 주지 말고, 설계 결정의 이유를 설명하세요.

---

## 임무별 힌트 전략

### 임무 1: User와 Room 기본 구조

**자주 묻는 질문**:

Q: "User와 Room이 서로 참조하는데, 왜 둘 다 Weak인가요?"
→ "User가 Room을 Rc로, Room도 User를 Rc로 참조하면 어떻게 될까요? 둘 다 drop되지 않습니다."

Q: "broadcast()에서 Weak::upgrade()가 None이면?"
→ "그 User가 이미 drop된 것입니다. members에서 정리하는 게 좋겠죠?"

Q: "room_count()와 member_count()가 다르게 나와요"
→ "Weak 참조 중 Some(Rc)가 되는 것만 세야 합니다. dead weak는 건너뛰세요."

**확인 질문**:
- Weak::upgrade()는 언제 None을 반환하나요?
- join_room()과 add_member()를 둘 다 호출해야 하는 이유는?

---

### 임무 2: SingleThreadChatServer

**자주 묻는 질문**:

Q: "HashMap에 저장하면 왜 Rc가 필요한가요?"
→ "User가 Room도 참조하고, Room도 User를 참조합니다. 여러 곳에서 같은 데이터를 공유하려면?"

Q: "join_room()에서 user와 room 둘 다 접근해야 하는데..."
→ "먼저 get으로 Rc를 얻고, 그 다음 각각의 borrow()를 호출하세요. 동시에 borrow_mut()하지 않도록 주의!"

Q: "send_message()가 복잡해요"
→ "단계별로: 1) user 찾기 2) room 찾기 3) Message 생성 4) room.broadcast() 호출"

**확인 질문**:
- create_user()에서 ID는 어떻게 생성하나요?
- 존재하지 않는 room_id로 join하면?

---

### 임무 3: MultiThreadChatServer

**자주 묻는 질문**:

Q: "왜 ThreadSafeUser와 ThreadSafeRoom을 새로 만드나요?"
→ "RefCell은 스레드 안전하지 않습니다. Mutex나 RwLock으로 교체해야 해요."

Q: "Arc<RwLock<HashMap<..., Arc<...>>>>가 너무 복잡해요"
→ "바깥에서 안으로: 1) Arc로 HashMap 공유 2) RwLock으로 HashMap 보호 3) 내부 값도 Arc로 공유"

Q: "create_user()에서 write lock을 얻어야 하나요?"
→ "네, HashMap에 insert하니까요. 읽기만 할 때는 read()로 충분합니다."

Q: "send_message()에서 deadlock이 날 것 같아요"
→ "락 순서를 일관되게 유지하세요. 예: 항상 users → rooms 순서로."

**확인 질문**:
- RwLock의 read()와 write()의 차이는?
- clone_server()가 데이터를 복사하나요?

---

### 임무 4: MessageBroker

**자주 묻는 질문**:

Q: "워커 스레드에서 server를 어떻게 접근하나요?"
→ "new()에서 server를 move로 워커 스레드로 옮기세요. MultiThreadChatServer는 Clone할 수 있으니까요."

Q: "shutdown()에서 스레드가 끝나기를 어떻게 기다리나요?"
→ "Shutdown 커맨드를 보내면 워커가 루프를 탈출합니다. 그 다음 handle.join()으로 대기."

Q: "sender를 clone하지 않고 다른 스레드에서 쓰려면?"
→ "MessageBroker 자체가 sender를 소유하고 있어서, send_command(&self)에서 &self.sender.send()할 수 있습니다."

**확인 질문**:
- mpsc 채널에서 tx.send()가 Err를 반환하면?
- rx.recv()가 Err를 반환하면?

---

### 임무 5: StatsCollector

**자주 묻는 질문**:

Q: "record_message()에서 read()로 충분하지 않나요?"
→ "값을 수정하니까 write()가 필요합니다."

Q: "AtomicU64를 쓰면 안 되나요?"
→ "가능합니다! 더 효율적이에요. 하지만 여기선 RwLock 연습이 목표입니다."

Q: "get_stats()는 read()로 되나요?"
→ "네, 읽기만 하니까요. Clone해서 반환하세요."

**확인 질문**:
- 여러 스레드에서 동시에 record_message()하면?
- RwLock이 Mutex보다 좋은 상황은?

---

## 공통 실수 대응

### Deadlock 패턴

```rust
// 잘못된 코드 - deadlock 위험!
let users = self.users.write().unwrap();
let rooms = self.rooms.write().unwrap();
// 다른 스레드가 rooms → users 순으로 락을 잡으면 deadlock!

// 올바른 코드 - 락 순서 일관성
// 항상 users → rooms 순서로, 또는 락 범위 최소화
```

→ "락 순서가 일관된가요? 또는 락 범위를 줄일 수 있나요?"

### 불필요한 Clone

```rust
// 비효율적
let users = self.users.read().unwrap();
let user = users.get(&id).cloned();  // Arc를 clone
drop(users);  // 락 해제
user

// 효율적
let users = self.users.read().unwrap();
users.get(&id).cloned()  // 락 범위 내에서 clone 후 바로 반환
```

→ "락을 잡은 상태에서 바로 반환할 수 있나요?"

### Weak 정리 누락

```rust
// 문제: dead weak가 계속 쌓임
fn broadcast(&self, msg: Message) {
    for weak in self.members.borrow().iter() {
        if let Some(user) = weak.upgrade() {
            // ...
        }
        // dead weak는 그대로 남음
    }
}

// 해결: 주기적으로 정리 또는 broadcast 시 정리
fn broadcast(&self, msg: Message) {
    let mut members = self.members.borrow_mut();
    members.retain(|weak| weak.upgrade().is_some());
    // ...
}
```

→ "dead weak를 언제 정리하고 있나요?"

---

## 점진적 힌트 레벨

### Level 1: 개념 질문
"싱글 스레드와 멀티 스레드에서 공유의 차이가 뭔가요?"

### Level 2: 타입 힌트
"Arc<RwLock<HashMap<...>>>이 필요합니다."

### Level 3: 메서드 힌트
"read().unwrap()으로 읽기 락을 얻으세요."

### Level 4: 구조 힌트
```rust
pub fn get_user(&self, id: UserId) -> Option<Arc<ThreadSafeUser>> {
    self.users.read().unwrap().get(&id).cloned()
}
```

---

## 절대 하지 말 것

1. 전체 구현 코드 제공
2. todo!() 대신 쓸 코드 작성
3. 테스트 통과 코드 복사-붙여넣기

---

## 격려 메시지

- "채팅 서버는 실제로 이런 패턴들을 사용합니다. 좋은 연습이에요."
- "컴파일러가 데이터 레이스를 막아주니까 안심하고 실험하세요."
- "rust-smartptr-dojo에서 개별 패턴을 연습한 게 여기서 통합됩니다."
