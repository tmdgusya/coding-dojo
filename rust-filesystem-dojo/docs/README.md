# Rust FileSystem Dojo - fstools

## 목표
이 수련장은 **fstools** (파일 시스템 유틸리티)를 만들면서 Rust의 파일 I/O API를 마스터하기 위해 만들어졌습니다.
Roach 님은 Rust 기본 문법을 익힌 상태이므로, 실제 파일 시스템을 다루는 실용적인 프로젝트를 통해 학습합니다.

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
# ... mission_7까지
cargo test final_mission
```

---

## 임무 1: 기본 파일 읽기 (Basic File Reading)

### 설명
Rust에서 파일을 읽는 가장 간단한 방법을 학습합니다.
`std::fs::read_to_string`과 `std::fs::read`를 사용합니다.

### 요구사항
1. `read_file_to_string` 함수 구현
   - 파일 경로를 받아 내용을 `String`으로 반환
   - `io::Result<String>` 타입 사용
2. `read_file_to_bytes` 함수 구현
   - 파일 내용을 `Vec<u8>`로 반환

### 예상 출력 결과
```rust
let content = read_file_to_string("hello.txt")?;
println!("{}", content);  // "Hello, Rust!"

let bytes = read_file_to_bytes("hello.txt")?;
println!("{:?}", bytes);  // [72, 101, 108, 108, 111, ...]
```

### 핵심 개념
- `std::fs::read_to_string`
- `std::fs::read`
- `io::Result<T>` 에러 타입
- `?` 연산자

---

## 임무 2: 기본 파일 쓰기 (Basic File Writing)

### 설명
파일에 내용을 쓰는 기본 방법을 학습합니다.
덮어쓰기와 추가(append) 모드의 차이를 이해합니다.

### 요구사항
1. `write_string_to_file` 함수 구현
   - 문자열을 파일에 쓰기 (기존 내용 덮어쓰기)
2. `append_to_file` 함수 구현
   - 기존 파일 끝에 내용 추가
   - `OpenOptions`를 사용

### 예상 출력 결과
```rust
write_string_to_file("output.txt", "Hello")?;
// output.txt: "Hello"

write_string_to_file("output.txt", "World")?;
// output.txt: "World" (덮어쓰기)

append_to_file("log.txt", "Line 1\n")?;
append_to_file("log.txt", "Line 2\n")?;
// log.txt: "Line 1\nLine 2\n"
```

### 핵심 개념
- `std::fs::write`
- `std::fs::OpenOptions`
- `.create(true)`, `.append(true)`, `.write(true)`

---

## 임무 3: File 구조체와 저수준 I/O

### 설명
`std::fs::File` 구조체를 직접 다루고, `Read`/`Write` 트레이트를 이해합니다.
청크 단위 읽기를 통해 대용량 파일 처리의 기초를 배웁니다.

### 요구사항
1. `read_with_file_struct` 함수 구현
   - `File::open`과 `read_to_string` 메서드 사용
2. `write_with_file_struct` 함수 구현
   - `File::create`와 `write_all` 메서드 사용
3. `read_in_chunks` 함수 구현
   - 지정된 크기의 청크 단위로 파일 읽기

### 예상 출력 결과
```rust
let chunks = read_in_chunks("large_file.txt", 1024)?;
for chunk in chunks {
    process(chunk);
}
```

### 핵심 개념
- `std::fs::File`
- `std::io::Read` 트레이트
- `std::io::Write` 트레이트
- `read_to_string(&mut self)` vs `fs::read_to_string(path)`

---

## 임무 4: 버퍼링된 I/O (Buffered I/O)

### 설명
`BufReader`와 `BufWriter`를 사용하여 효율적인 I/O를 구현합니다.
라인 단위 처리와 grep 기능을 구현합니다.

### 요구사항
1. `read_lines` 함수 구현
   - 파일을 라인 단위로 읽어 `Vec<String>` 반환
2. `grep_lines` 함수 구현
   - 패턴이 포함된 라인과 라인 번호를 반환
3. `write_lines` 함수 구현
   - 여러 줄을 효율적으로 파일에 쓰기

### 예상 출력 결과
```rust
let lines = read_lines("poem.txt")?;
// ["The Road Not Taken", "by Robert Frost", "", "Two roads..."]

let matches = grep_lines("poem.txt", "road")?;
// [(4, "Two roads diverged in a yellow wood,")]
```

### 핵심 개념
- `std::io::BufReader`
- `std::io::BufWriter`
- `BufRead::lines()` 이터레이터
- 버퍼링의 이점 (시스템 콜 감소)

---

## 임무 5: Path와 PathBuf

### 설명
플랫폼 독립적인 경로 처리 방법을 학습합니다.
Windows와 Unix의 경로 차이를 추상화합니다.

### 요구사항
1. `get_filename` - 경로에서 파일명 추출
2. `get_extension` - 확장자 추출
3. `change_extension` - 확장자 변경
4. `join_paths` - 경로 결합
5. `is_absolute_path` - 절대 경로 확인

### 예상 출력 결과
```rust
get_filename("/home/user/file.txt")  // Some("file.txt")
get_extension("archive.tar.gz")      // Some("gz")
change_extension("doc.txt", "pdf")   // PathBuf("doc.pdf")
join_paths("/home", "user")          // PathBuf("/home/user")
is_absolute_path("/home")            // true
is_absolute_path("relative")         // false
```

### 핵심 개념
- `std::path::Path` (불변 참조, `&str`과 유사)
- `std::path::PathBuf` (소유, `String`과 유사)
- `Path::file_name()`, `extension()`, `parent()`
- `PathBuf::push()`, `set_extension()`

---

## 임무 6: 디렉토리 순회 (Directory Traversal)

### 설명
디렉토리 내용을 읽고 재귀적으로 탐색하는 방법을 학습합니다.

### 요구사항
1. `list_directory` - 디렉토리 엔트리 나열
2. `list_files` - 파일만 나열 (디렉토리 제외)
3. `find_files_by_extension` - 특정 확장자 파일 찾기
4. `walk_directory` - 재귀적 디렉토리 순회

### 예상 출력 결과
```rust
let entries = list_directory("src")?;
// [PathBuf("src/lib.rs"), PathBuf("src/main.rs")]

let txt_files = find_files_by_extension("docs", "txt")?;
// [PathBuf("docs/notes.txt"), PathBuf("docs/readme.txt")]

let all_files = walk_directory("project")?;
// 모든 하위 디렉토리의 파일들
```

### 핵심 개념
- `std::fs::read_dir`
- `DirEntry` 구조체
- 재귀 함수 / 스택 기반 순회
- `Iterator` 활용

---

## 임무 7: 파일 메타데이터 (File Metadata)

### 설명
파일의 속성과 메타데이터를 다루는 방법을 학습합니다.

### 요구사항
1. `FileInfo` 구조체와 `from_path` 구현
2. `get_file_size` - 파일 크기 반환
3. `is_file` - 파일 여부 확인
4. `is_directory` - 디렉토리 여부 확인
5. `path_exists` - 경로 존재 확인

### 예상 출력 결과
```rust
let info = FileInfo::from_path("file.txt")?;
// FileInfo { path: "file.txt", size: 1024, is_file: true, ... }

get_file_size("file.txt")?  // 1024
is_file("file.txt")         // true
is_directory("src")         // true
path_exists("missing.txt")  // false
```

### 핵심 개념
- `std::fs::metadata`
- `Metadata` 구조체
- `metadata.len()`, `is_file()`, `is_dir()`
- `Path::exists()`

---

## 최종 임무: 통합 파일 유틸리티

### 설명
지금까지 배운 모든 것을 통합하여 실용적인 파일 유틸리티를 구현합니다.

### 요구사항
1. `copy_file` - 파일 복사
2. `move_file` - 파일 이동
3. `remove_dir_recursive` - 재귀적 디렉토리 삭제
4. `replace_in_file` - 파일 내 문자열 치환
5. `calculate_dir_size` - 디렉토리 총 크기 계산

### 예상 출력 결과
```rust
copy_file("src.txt", "dst.txt")?;       // 복사된 바이트 수 반환
move_file("old.txt", "new.txt")?;       // 원본 삭제됨
remove_dir_recursive("temp_dir")?;      // 디렉토리와 내용 모두 삭제
replace_in_file("file.txt", "old", "new")?;  // 치환 횟수 반환
calculate_dir_size("project")?;          // 총 바이트 수 반환
```

### 핵심 개념
- `std::fs::copy`
- `std::fs::rename`
- `std::fs::remove_file`, `remove_dir_all`
- 지금까지 배운 모든 것의 조합

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
cargo test mission_6
cargo test mission_7
cargo test final_mission

# 프로그램 실행
cargo run -- read tests/fixtures/hello.txt
cargo run -- lines tests/fixtures/poem.txt
cargo run -- grep "road" tests/fixtures/poem.txt
cargo run -- ls tests/fixtures
cargo run -- info tests/fixtures/hello.txt

# Release 빌드
cargo build --release
```

## 참고 자료
- [The Rust Book - File I/O](https://doc.rust-lang.org/book/ch12-02-reading-a-file.html)
- [std::fs Documentation](https://doc.rust-lang.org/std/fs/index.html)
- [std::io Documentation](https://doc.rust-lang.org/std/io/index.html)
- [std::path Documentation](https://doc.rust-lang.org/std/path/index.html)
