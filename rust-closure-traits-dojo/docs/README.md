# Rust Closure & Traits Dojo

`Box<dyn FnOnce() + Send + 'static>`을 완전히 이해하기 위한 수련장입니다.

## 이 dojo를 마치면

```rust
Box<dyn FnOnce() + Send + 'static>
```

이 타입을 보고 바로 이해할 수 있습니다:
- 힙에 저장된 클로저
- 한 번만 호출 가능
- 다른 스레드로 전송 가능
- 소유된 데이터만 캡처

## 학습 경로

| 임무 | 주제 | 핵심 개념 |
|------|------|----------|
| 1 | 클로저 트레이트 | `Fn`, `FnMut`, `FnOnce` |
| 2 | 트레이트 객체 | `dyn Trait`, `Box<dyn Trait>` |
| 3 | 생명주기 심화 | `'static`의 두 가지 의미 |
| 4 | 마커 트레이트 | `Send`, `Sync` |
| 5 | **통합** | `Box<dyn FnOnce() + Send + 'static>` |

## 빠른 시작

```bash
cd rust-closure-traits-dojo

cargo test             # 전체 테스트
cargo test mission_1   # Fn, FnMut, FnOnce
cargo test mission_2   # dyn Trait
cargo test mission_3   # 'static
cargo test mission_4   # Send + Sync
cargo test mission_5   # 통합
```

## 임무 상세

### 임무 1: Fn, FnMut, FnOnce

**핵심 질문**: 왜 클로저 트레이트가 3개일까?

```
캡처 방식에 따라 결정됨:

FnOnce: 값을 소비 (move)     → 한 번만 호출 가능
FnMut:  값을 변경 (&mut)     → 여러 번 호출, &mut self 필요
Fn:     값을 읽기 (&)        → 여러 번 호출, &self로 충분

포함 관계: Fn ⊂ FnMut ⊂ FnOnce
```

---

### 임무 2: dyn Trait

**핵심 질문**: 제네릭과 트레이트 객체의 차이는?

```
제네릭 (정적 디스패치):
- 컴파일 타임에 타입 결정
- 각 타입별로 코드 생성 (monomorphization)
- 빠르지만 바이너리 크기 증가

트레이트 객체 (동적 디스패치):
- 런타임에 타입 결정
- vtable을 통한 메서드 호출
- 유연하지만 약간의 오버헤드
```

---

### 임무 3: 'static

**핵심 질문**: 'static은 "영원히 살아있다"는 뜻?

```
두 가지 의미:

1. &'static str
   → 프로그램 전체 동안 유효한 참조
   → 예: 문자열 리터럴

2. T: 'static
   → T가 빌린 참조를 포함하지 않음
   → 또는 포함하더라도 'static 참조만
   → String, i32, Vec<T> 모두 만족!
```

---

### 임무 4: Send와 Sync

**핵심 질문**: 왜 Rc는 스레드로 보낼 수 없을까?

```
Send: 다른 스레드로 소유권 이동 가능
Sync: 여러 스레드에서 &T로 접근 가능

Rc가 Send가 아닌 이유:
→ 참조 카운트가 atomic이 아님
→ 동시 접근 시 데이터 레이스

해결: Arc 사용 (Atomic Reference Count)
```

---

### 임무 5: 통합

모든 조각을 맞춥니다:

```rust
Box<dyn FnOnce() + Send + 'static>
│    │   │         │      │
│    │   │         │      └── 빌린 참조 없음 (임무 3)
│    │   │         └── 스레드로 전송 가능 (임무 4)
│    │   └── 한 번만 호출 (임무 1)
│    └── 트레이트 객체 (임무 2)
└── 힙 저장 (rust-worker-dojo 임무 1)
```

이것이 바로 **Worker Pool의 Job 타입**입니다!

## 다음 단계

이 dojo 완료 후 → rust-worker-dojo 임무 6.5, 7로 복귀
