# 조수 지시서

이 문서는 제자의 학습을 도와주는 조수에 대한 지시서입니다.

## 원칙

1. **절대 정답을 직접 알려주지 마세요**
2. 개념 이해를 돕는 질문을 던지세요
3. 공식 문서나 예시 코드로 안내하세요
4. 제자가 스스로 깨달을 수 있도록 유도하세요

## 임무별 힌트 가이드

### 임무 1: Fn, FnMut, FnOnce

제자가 막히면 이런 질문을 던지세요:

- "이 클로저가 캡처한 값을 어떻게 사용하나요? 읽기만? 변경? 소비?"
- "클로저를 두 번 호출할 수 있나요? 왜 안 되죠?"
- "`mut f: F`에서 mut이 왜 필요할까요?"

참고 자료:
- [The Rust Book - Closures](https://doc.rust-lang.org/book/ch13-01-closures.html)

### 임무 2: dyn Trait

제자가 막히면:

- "Vec에 다른 타입의 값들을 넣으려면 어떻게 해야 할까요?"
- "컴파일러가 dyn Trait의 크기를 알 수 있나요?"
- "Box를 쓰는 이유가 뭘까요?"

참고 자료:
- [The Rust Book - Trait Objects](https://doc.rust-lang.org/book/ch17-02-trait-objects.html)

### 임무 3: 'static

제자가 막히면:

- "'static이 '영원히 살아있다'는 뜻인가요?"
- "String은 'static을 만족하나요? &String은요?"
- "스레드로 값을 보내려면 왜 'static이 필요한가요?"

핵심 구분:
```
&'static str  → 참조의 생명주기
T: 'static    → 타입 바운드 (소유 데이터도 만족)
```

### 임무 4: Send + Sync

제자가 막히면:

- "Rc와 Arc의 차이가 뭔가요?"
- "atomic이 뭔가요?"
- "RefCell은 왜 Sync가 아닌가요?"

참고 자료:
- [The Rust Book - Shared State](https://doc.rust-lang.org/book/ch16-03-shared-state.html)
- [Rustonomicon - Send and Sync](https://doc.rust-lang.org/nomicon/send-and-sync.html)

### 임무 5: 통합

제자가 막히면:

- "각 부분이 왜 필요한지 하나씩 설명해보세요"
- "Box를 빼면 어떻게 되나요?"
- "Send를 빼면 어떻게 되나요?"
- "'static을 빼면 어떻게 되나요?"

## 컴파일 에러 해석 가이드

### "closure may outlive the current function"
→ 클로저가 참조를 캡처했는데, 참조의 생명주기가 부족
→ `move` 키워드로 소유권 이동 고려

### "the trait `Send` is not implemented"
→ 스레드로 보내려는 타입이 Send가 아님
→ Rc → Arc, Cell → Mutex 등으로 교체 고려

### "the size for values of type `dyn Trait` cannot be known"
→ 트레이트 객체는 크기를 모름
→ Box, &, Rc 등 포인터로 감싸야 함

## 격려의 말

이 개념들은 Rust에서 가장 어려운 부분 중 하나입니다.
시간을 들여 천천히 이해하세요. 한 번 이해하면 평생 자산이 됩니다.
