# self vs &self vs &mut self - 소유권의 3가지 얼굴

> **조교의 한마디**: 이 문서는 Rust의 메서드 시그니처에서 가장 중요한 개념인 `self`, `&self`, `&mut self`의 차이를 마스터하기 위한 가이드입니다.

---

## 📊 한눈에 보는 비교

| | `self` | `&mut self` | `&self` |
|---|--------|-------------|---------|
| **소유권** | 가져감 (소비) | 빌림 (수정) | 빌림 (읽기) |
| **호출 후 사용** | ❌ 불가능 | ✅ 가능 | ✅ 가능 |
| **용도** | 변환/소비 | 내용 수정 | 읽기 |
| **라이프타임** | 독립 | 의존 | 의존 |
| **호출 횟수** | 1번 | 무한 | 무한 |

---

## 🎯 핵심 차이: 호출 후 사용 가능 여부

```rust
let mut v = vec![1, 2, 3];

// &mut self - 수정 후에도 사용 가능!
v.push(4);          // &mut self
v.push(5);          // &mut self (계속 가능!)
println!("{:?}", v);  // ✅ v 여전히 사용 가능!

// self - 소비되어 사라짐!
let v = vec![1, 2, 3];
let iter = v.into_iter();  // self로 받음
// println!("{:?}", v);    // ❌ v는 사라짐!
```

---

## 💡 실전 예시

### 예시 1: String의 메서드들

```rust
let mut s = String::from("hello");

// &mut self - 수정 (계속 사용 가능)
s.push_str(" world");  // &mut self
s.push('!');           // &mut self
println!("{}", s);     // ✅ "hello world!"

// self - 소비 (더 이상 사용 불가)
let s = String::from("hello");
let bytes = s.into_bytes();  // self
// println!("{}", s);        // ❌ s는 없어졌음!
println!("{:?}", bytes);     // [104, 101, 108, 108, 111]
```

**차이점**:
- `push_str(&mut self)`: String을 **수정**하지만 **계속 소유**
- `into_bytes(self)`: String을 **소비**하고 `Vec<u8>`로 **변환**

---

### 예시 2: Vec의 메서드들

```rust
let mut v = vec![1, 2, 3];

// &mut self - 수정만
v.push(4);      // &mut self - Vec에 추가
v.push(5);      // &mut self - 계속 사용 가능!
v.sort();       // &mut self - 정렬
println!("{:?}", v);  // ✅ [1, 2, 3, 4, 5]

// self - 소비
let v = vec![1, 2, 3];
for item in v.into_iter() {  // self - Vec 소비
    println!("{}", item);
}
// println!("{:?}", v);  // ❌ v는 없음!
```

---

## 🤔 왜 이렇게 구분할까?

### 잘못된 설계 예시

```rust
// ❌ 만약 into_bytes()를 &mut self로 만든다면?
impl String {
    fn into_bytes(&mut self) -> Vec<u8> {
        // String의 내용을 Vec<u8>로 반환하면
        // String은 빈 문자열이 되어야 함?
        // bytes는 복사본? 아니면 원본?
        // 혼란스러움!
    }
}
```

### 올바른 설계

```rust
// ✅ self로 받으면 명확
impl String {
    fn into_bytes(self) -> Vec<u8> {
        // String 전체를 소비하고
        // 그 내부 바이트를 반환
        // 복사 없음, 소유권 이동!
    }
}

let s = String::from("hello");
let bytes = s.into_bytes();
// s는 없어짐 - 명확!
// bytes는 s의 데이터를 소유 - 명확!
```

---

## 📖 용도별 메서드 설계 가이드

### `&self` - 읽기 전용

```rust
impl Vec<i32> {
    fn len(&self) -> usize {
        // Vec의 길이만 반환
        // Vec는 그대로
    }
    
    fn is_empty(&self) -> bool {
        // 비어있는지 확인만
        // Vec는 그대로
    }
    
    fn get(&self, index: usize) -> Option<&i32> {
        // 참조 반환
        // Vec는 그대로
    }
}

// 사용
let v = vec![1, 2, 3];
println!("{}", v.len());       // v는 그대로
println!("{}", v.is_empty());  // v는 그대로
println!("{:?}", v.get(0));    // v는 그대로
```

---

### `&mut self` - 같은 타입, 수정만

```rust
impl Vec<i32> {
    fn push(&mut self, value: i32) {
        // Vec는 그대로, 요소만 추가
    }
    
    fn pop(&mut self) -> Option<i32> {
        // Vec는 그대로, 요소만 제거
    }
    
    fn clear(&mut self) {
        // Vec는 그대로, 내용만 비움
    }
    
    fn sort(&mut self) {
        // Vec는 그대로, 순서만 변경
    }
}

// 사용
let mut v = vec![3, 1, 2];
v.push(4);    // v는 그대로
v.pop();      // v는 그대로
v.sort();     // v는 그대로
println!("{:?}", v);  // ✅ v 여전히 존재!
```

**공통점**: 
- 모두 Vec 타입 유지
- 내용만 수정
- 계속 사용 가능

---

### `self` - 타입 변환 또는 완전 소비

```rust
impl Vec<i32> {
    fn into_iter(self) -> IntoIter<i32> {
        // Vec → IntoIter로 변환
        // Vec는 소비됨
    }
    
    fn into_boxed_slice(self) -> Box<[i32]> {
        // Vec → Box<[i32]>로 변환
        // Vec는 소비됨
    }
}

// 사용
let v = vec![1, 2, 3];
let iter = v.into_iter();  // v는 없어짐
// println!("{:?}", v);    // ❌

let v = vec![1, 2, 3];
let boxed = v.into_boxed_slice();  // v는 없어짐
// println!("{:?}", v);            // ❌
```

**공통점**:
- 타입이 변환됨
- 원본 소비됨
- 한 번만 호출 가능

**네이밍 컨벤션**: `into_*` 메서드는 거의 항상 `self`를 받음!

---

## 🎯 JoinHandle 케이스 스터디

### 왜 join()은 self를 받을까?

```rust
impl<T> JoinHandle<T> {
    // &self - 읽기만, 계속 사용 가능
    pub fn thread(&self) -> &Thread {
        &self.thread
    }
    
    // ✅ self로 받음 - 명확!
    pub fn join(self) -> Result<T> {
        // JoinHandle 전체를 소비
        // T를 꺼내서 반환
        // JoinHandle은 사라짐
    }
}
```

**만약 &mut self였다면?**

```rust
// ❌ 문제점
let mut handle = thread::spawn(|| 42);

let result = handle.join();  // result = 42
// handle은 아직 있음
// 하지만 내부는 비어있음?

handle.join();  // 💥 또 호출? 뭘 반환?
                // None? 에러? 혼란!

// ✅ self로 받으면:
let handle = thread::spawn(|| 42);
let result = handle.join();  // result = 42
// handle.join();  // ❌ 컴파일 에러! 명확!
```

**핵심**: 
- `join()`은 한 번만 호출되어야 함
- 결과값 `T`는 한 번만 받을 수 있음
- `self`로 받아서 **타입 시스템이 강제**!

---

## 🔍 라이프타임 관점

### &self와 &mut self - 라이프타임 의존

```rust
impl String {
    fn get_reference(&self) -> &str {
    //               ^^^^^        ^^^^ 
    //               빌림         self와 같은 라이프타임!
        &self[..]
    }
}

// 문제 상황
fn bad_example() -> &str {
    let s = String::from("hello");
    s.get_reference()  // 💥 에러!
}  // ← s가 여기서 drop!

// s는 사라지는데, 참조를 반환?
```

---

### self - 라이프타임 독립

```rust
impl Vec<i32> {
    fn into_first(self) -> Option<i32> {
    //            ^^^^              ^^^
    //            소유권            값 자체 반환 (참조 아님!)
        self.into_iter().next()
    }
}

// OK!
fn works() -> Option<i32> {
    let v = vec![1, 2, 3];
    v.into_first()  // ✅ i32 반환 (독립적)
}  // v는 사라지지만, i32 값은 반환됨
```

**차이점**:
- `&self`: 반환값이 `self`에 **의존**
- `self`: 반환값이 `self`와 **독립**

---

## 📚 실전 패턴 정리

### 패턴 1: Builder 패턴 (self 체이닝)

```rust
struct Request {
    url: String,
    method: String,
    headers: Vec<String>,
}

impl Request {
    fn new(url: String) -> Self {
        Request {
            url,
            method: "GET".to_string(),
            headers: vec![],
        }
    }
    
    // self를 받아서 수정 후 self 반환!
    fn method(mut self, method: String) -> Self {
        self.method = method;
        self  // 소유권 반환
    }
    
    fn header(mut self, header: String) -> Self {
        self.headers.push(header);
        self  // 소유권 반환
    }
}

// 사용
let req = Request::new("https://api.com".to_string())
    .method("POST".to_string())
    .header("Content-Type: json".to_string())
    .header("Auth: token".to_string());
```

**왜 self?**
- 각 메서드가 소유권을 받아서 수정하고 반환
- 체이닝 가능
- 불변성 유지 (원본은 소비됨)

---

### 패턴 2: 상태 전환 (Type State Pattern)

```rust
struct Disconnected;
struct Connected;

struct Connection<State> {
    address: String,
    state: std::marker::PhantomData<State>,
}

impl Connection<Disconnected> {
    // self를 소비하고 다른 타입 반환!
    fn connect(self) -> Connection<Connected> {
        Connection {
            address: self.address,
            state: std::marker::PhantomData,
        }
    }
}

impl Connection<Connected> {
    fn send_data(&mut self, data: &str) {
        // Connected 상태에서만 호출 가능
    }
}

// 사용
let conn = Connection::<Disconnected> {
    address: "127.0.0.1".to_string(),
    state: std::marker::PhantomData,
};

let mut conn = conn.connect();  // 타입 변환!
conn.send_data("hello");
```

**왜 self?**
- 타입 레벨에서 상태 변경 강제
- `Disconnected`에서는 `send_data()` 호출 불가 (컴파일 에러!)

---

## ✅ 언제 무엇을 쓸까? - 의사결정 트리

```
메서드를 만들 때:

1. 값을 변경하나요?
   NO  → &self (읽기만)
   YES → 2번으로

2. 타입이 바뀌나요?
   YES → self (변환/소비)
   NO  → 3번으로

3. 원본을 계속 쓰나요?
   YES → &mut self (수정)
   NO  → self (소비)
```

---

## 🧪 연습 문제

### 문제 1: 다음 메서드들의 시그니처를 추론하세요

```rust
impl String {
    // 1. 문자열 길이 반환
    fn len(???) -> usize { }
    
    // 2. 문자열 끝에 추가
    fn push_str(???, s: &str) { }
    
    // 3. String → Vec<u8> 변환
    fn into_bytes(???) -> Vec<u8> { }
    
    // 4. 모든 문자를 대문자로 변경
    fn make_uppercase(???) { }
    
    // 5. 대문자 버전의 새 String 반환
    fn to_uppercase(???) -> String { }
}
```

<details>
<summary>정답</summary>

```rust
fn len(&self) -> usize { }              // 읽기만
fn push_str(&mut self, s: &str) { }     // 수정
fn into_bytes(self) -> Vec<u8> { }      // 변환 (소비)
fn make_uppercase(&mut self) { }        // 수정
fn to_uppercase(&self) -> String { }    // 읽기 + 새 값 생성
```
</details>

---

### 문제 2: 버그 찾기

```rust
impl<T> JoinHandle<T> {
    pub fn join(mut self) -> Result<T> {
        // 왜 mut self를 쓸까요?
        // mut이 필요한가요?
    }
}
```

<details>
<summary>답</summary>

`mut self`는 불필요합니다!
- `self`로 받으면 소유권을 가져감
- 내부를 수정하려면 `mut`가 필요하지만
- 어차피 소비되므로 `mut` 없이 내부 변경 가능
- 실제로는 `self`만으로 충분!

정답: `pub fn join(self) -> Result<T>`
</details>

---

## 🎓 핵심 정리

**기억해야 할 3가지**:

1. **`&self`**: 읽기만 → 계속 사용 가능
2. **`&mut self`**: 수정만 → 계속 사용 가능 (같은 타입 유지)
3. **`self`**: 소비/변환 → 한 번만 사용 (타입 변환 가능)

**네이밍 컨벤션**:
- `into_*`: 거의 항상 `self` (변환)
- `to_*`: 거의 항상 `&self` (복사본 생성)
- `as_*`: 거의 항상 `&self` (참조 변환)

**설계 원칙**:
```
최소 권한으로 빌려라!
- 읽기만? → &self
- 수정? → &mut self
- 소비/변환? → self
```

---

## 📚 더 깊이 알아보기

- [Rust Book - Method Syntax](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)
- [Rust Reference - Methods](https://doc.rust-lang.org/reference/items/associated-items.html#methods)
- [API Guidelines - Ownership](https://rust-lang.github.io/api-guidelines/interoperability.html#c-ownership)

---

**마스터의 한마디**: 
> "소유권을 이해하는 것이 Rust의 시작이고, self의 선택을 마스터하는 것이 Rust의 완성입니다."
