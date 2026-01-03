# Rust FileSystem Dojo - fstools

Rust의 파일 시스템 API를 배우기 위한 수련장입니다. **fstools** (파일 유틸리티)를 만들면서 파일 I/O의 마스터가 되어보세요.

## 빠른 시작

```bash
cd rust-filesystem-dojo

# 테스트 실행 (현재 모두 실패 상태)
cargo test

# 특정 임무만 테스트
cargo test mission_1    # 기본 파일 읽기
cargo test mission_2    # 기본 파일 쓰기
cargo test mission_3    # File 구조체
cargo test mission_4    # 버퍼링된 I/O
cargo test mission_5    # Path와 PathBuf
cargo test mission_6    # 디렉토리 순회
cargo test mission_7    # 메타데이터
cargo test final        # 통합 유틸리티
```

## 학습 경로

| 임무 | 주제 | 핵심 개념 |
|------|------|----------|
| 1 | 기본 파일 읽기 | `fs::read_to_string`, `fs::read` |
| 2 | 기본 파일 쓰기 | `fs::write`, `OpenOptions` |
| 3 | File 구조체 | `File::open`, `Read`/`Write` 트레이트 |
| 4 | 버퍼링된 I/O | `BufReader`, `BufWriter`, `lines()` |
| 5 | Path와 PathBuf | 플랫폼 독립적 경로 처리 |
| 6 | 디렉토리 순회 | `read_dir`, 재귀적 탐색 |
| 7 | 파일 메타데이터 | `metadata`, 파일 속성 |
| 최종 | 통합 유틸리티 | `copy`, `rename`, `remove_dir_all` |

## 상세 안내

- 학습 계획: [docs/README.md](docs/README.md)
- 도움 요청 시: [docs/Assistance.md](docs/Assistance.md) 참고
