# Rust Dojo - minigrep

## 목표
이 수련장은 **minigrep** (간단한 파일 검색 도구)를 만들면서 Rust의 핵심 개념을 마스터하기 위해 만들어졌습니다.
Roach 님은 경험 있는 프로그래머로서, TOP-DOWN 방식으로 실제 동작하는 프로그램을 만들면서 Rust를 익히게 됩니다.

## 학습 방식: RED -> GREEN
각 임무에는 **실패하는 테스트 코드**가 미리 작성되어 있습니다.
- **RED**: 테스트가 실패하는 상태에서 시작
- **GREEN**: 구현을 완성하여 테스트를 통과시킴
- **REFACTOR**: 필요시 코드 개선

```bash
# 테스트 실행
cargo test

# 특정 임무 테스트만 실행
cargo test mission_1
cargo test mission_2
```

---

## 임무 1: 기본 문법과 소유권 (Ownership)

### 설명
파일을 읽어서 내용을 반환하는 기본 기능을 구현합니다.
이 과정에서 Rust의 **소유권(Ownership)**, **String vs &str**, **함수 반환값**을 익힙니다.

### 요구사항
1. `read_file` 함수를 구현하세요.
   - 파일 경로를 받아 파일 내용을 `String`으로 반환
   - 파일이 없으면 에러 메시지를 포함한 `String` 반환
2. 소유권 규칙을 이해하고, 왜 `String`을 반환하는지 생각해보세요.

### 예상 출력 결과
```rust
// 파일이 존재할 때
let contents = read_file("poem.txt");
assert!(contents.contains("I'm nobody"));

// 파일이 없을 때
let contents = read_file("nonexistent.txt");
assert!(contents.contains("Error"));
```

### 핵심 개념
- `String` vs `&str` (owned vs borrowed)
- `std::fs::read_to_string`
- 소유권 이동 (move semantics)

---

## 임무 2: 에러 처리 (Result & Option)

### 설명
Rust의 강력한 에러 처리 시스템인 `Result<T, E>`와 `Option<T>`을 사용합니다.
임무 1의 함수를 개선하여 적절한 에러 타입을 반환하도록 합니다.

### 요구사항
1. `read_file_safe` 함수를 구현하세요.
   - `Result<String, std::io::Error>` 타입 반환
   - `?` 연산자를 사용한 에러 전파
2. `find_first_match` 함수를 구현하세요.
   - 첫 번째로 매칭되는 라인을 `Option<&str>`로 반환

### 예상 출력 결과
```rust
// Result 사용
match read_file_safe("poem.txt") {
    Ok(contents) => println!("파일 내용: {}", contents),
    Err(e) => println!("에러: {}", e),
}

// Option 사용
let line = find_first_match("hello\nworld\nhello rust", "rust");
assert_eq!(line, Some("hello rust"));
```

### 핵심 개념
- `Result<T, E>` 타입
- `Option<T>` 타입
- `?` 연산자 (error propagation)
- `match` 표현식
- `unwrap()`, `expect()`, `unwrap_or()`

---

## 임무 3: 구조체와 메서드 (Struct & impl)

### 설명
검색 설정을 담는 `Config` 구조체를 만들고, 메서드를 구현합니다.
커맨드라인 인자를 파싱하여 `Config`를 생성합니다.

### 요구사항
1. `Config` 구조체를 정의하세요.
   - `query`: 검색할 문자열
   - `filename`: 검색할 파일명
   - `case_sensitive`: 대소문자 구분 여부
2. `Config::new` 또는 `Config::build` 메서드 구현
   - 커맨드라인 인자 `&[String]`을 받아 `Config` 생성
   - 인자가 부족하면 에러 반환

### 예상 출력 결과
```rust
let args = vec![
    String::from("minigrep"),
    String::from("needle"),
    String::from("haystack.txt"),
];
let config = Config::build(&args)?;
assert_eq!(config.query, "needle");
assert_eq!(config.filename, "haystack.txt");
```

### 핵심 개념
- `struct` 정의
- `impl` 블록
- 연관 함수 (associated functions) vs 메서드
- `Self` 키워드
- 빌더 패턴

---

## 임무 4: 트레이트와 제네릭 (Traits & Generics)

### 설명
검색 결과를 다양한 형식으로 출력할 수 있도록 트레이트를 정의합니다.
제네릭을 사용하여 유연한 함수를 작성합니다.

### 요구사항
1. `SearchResult` 트레이트를 정의하세요.
   - `format(&self) -> String` 메서드
2. `LineMatch` 구조체 구현
   - 라인 번호, 내용을 담음
   - `SearchResult` 트레이트 구현
3. 제네릭 함수 `print_results<T: SearchResult>` 구현

### 예상 출력 결과
```rust
let match1 = LineMatch::new(1, "Hello, world!");
let match2 = LineMatch::new(5, "Hello, Rust!");

println!("{}", match1.format());
// 출력: "[Line 1] Hello, world!"

print_results(vec![match1, match2]);
// 출력:
// [Line 1] Hello, world!
// [Line 5] Hello, Rust!
```

### 핵심 개념
- `trait` 정의와 구현
- 트레이트 바운드 (`T: Trait`)
- `impl Trait` 문법
- 기본 메서드 구현
- `derive` 매크로 (`Debug`, `Clone`, `PartialEq`)

---

## 임무 5: 생명주기 (Lifetimes)

### 설명
Rust의 가장 독특한 개념인 **생명주기(Lifetimes)**를 이해합니다.
참조를 반환하는 함수에서 생명주기 어노테이션을 올바르게 사용합니다.

### 요구사항
1. `search` 함수를 구현하세요.
   - 쿼리와 내용을 받아 매칭되는 라인들을 반환
   - 반환 타입: `Vec<&str>` (내용에서 빌린 참조)
   - 올바른 생명주기 어노테이션 필요
2. `search_case_insensitive` 함수 구현

### 예상 출력 결과
```rust
let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

let results = search("rust", contents);
assert_eq!(results, vec!["Trust me."]);

let results = search_case_insensitive("rust", contents);
assert_eq!(results, vec!["Rust:", "Trust me."]);
```

### 핵심 개념
- 생명주기 어노테이션 (`'a`)
- 생명주기 생략 규칙 (elision rules)
- 댕글링 참조 방지
- `'static` 생명주기

---

## 최종 임무: minigrep 완성

### 설명
모든 임무를 통합하여 완전히 동작하는 minigrep을 완성합니다.

### 요구사항
1. `run` 함수로 메인 로직 통합
2. 환경 변수 `IGNORE_CASE` 지원
3. 에러 메시지를 `stderr`로 출력
4. 실제 파일로 테스트

### 예상 출력 결과
```bash
$ cargo run -- to poem.txt
Are you nobody, too?
How dreary to be somebody!

$ IGNORE_CASE=1 cargo run -- to poem.txt
Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!
```

---

## 사용법

```bash
# 프로젝트 빌드
cargo build

# 테스트 실행 (현재 상태: RED)
cargo test

# 특정 임무만 테스트
cargo test mission_1
cargo test mission_2
cargo test mission_3
cargo test mission_4
cargo test mission_5

# 프로그램 실행
cargo run -- <검색어> <파일명>

# Release 빌드
cargo build --release
```

## 참고 자료
- [The Rust Programming Language Book - Chapter 12](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings)
