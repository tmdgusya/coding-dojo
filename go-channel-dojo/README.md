# Go Channel 마스터리 도장

Go의 Channel을 체계적으로 학습하는 TDD 기반 코딩 도장입니다.

## 🎯 목표

- Go Channel의 기초부터 고급 패턴까지 마스터
- 실제 프로젝트에서 활용할 수 있는 동시성 프로그래밍 실력养成
- TDD(RED → GREEN → REFACTOR) 방식의 체계적 학습

## 📚 미션 목록

| 순서 | 미션 | 주제 | 난이도 |
|------|------|------|--------|
| 1 | Mission 1 | Unbuffered Channel 기초 | 🔴 입문 |
| 2 | Mission 2 | Buffered Channel | 🟡 기초 |
| 3 | Mission 3 | Select Statement | 🟢 중급 |
| 4 | Mission 4 | Channel Closure | 🔵 중급 |
| 5 | Mission 5 | Pipeline Pattern | 🟣 고급 |
| 6 | Mission 6 | Fan-out/Fan-in Pattern | 🟤 고급 |
| 7 | Mission 7 | Timeout & Context | ⚫ 고급 |
| 8 | Mission 8 | Rate Limiting | ⬛ 마스터 |

## 🚀 시작하기

### 사전 요구사항

- Go 1.21 이상
- 기본적인 Go 문법 이해

### 설치 및 실행

```bash
# 도장 디렉토리로 이동
cd go-channel-dojo

# 의존성 설치
go mod tidy

# 전체 테스트 실행
go test ./...

# 특정 미션만 테스트
go test ./internal/mission_01 -v
go test ./internal/mission_02 -v
# ... etc
```

## 📖 학습 방식

### TDD 사이클

각 미션은 TDD 방식으로 진행됩니다:

1. **RED**: 실패하는 테스트 확인
2. **GREEN**: 테스트를 통과하도록 구현
3. **REFACTOR**: 코드 개선

### 미션 진행 방법

```bash
# 1. 미션 테스트 확인 (RED 상태)
go test ./internal/mission_01 -v

# 2. 구현 코드 확인
cat internal/mission_01/src/mission.go

# 3. 구현 작성 (todo 플레이스홀더 제거)
# ...
# 구현 완료 후

# 4. 테스트 실행 (GREEN 확인)
go test ./internal/mission_01 -v

# 5. 코드 리팩토링
```

## 📂 디렉토리 구조

```
go-channel-dojo/
├── cmd/
│   └── main.go               # 예제 실행 파일
├── internal/
│   ├── mission_01/           # 미션별 코드
│   │   ├── src/
│   │   │   └── mission.go    # 구현
│   │   └── test/
│   │       └── mission_test.go # 테스트
│   ├── mission_02/
│   │   └── ...
│   └── ... (8개 미션)
├── test/
│   └── integration_test.go   # 통합 테스트
├── docs/
│   ├── README.md             # 도장 설명서
│   └── Assistance.md         # 조수 지시서 (힌트)
├── go.mod
├── go.sum
└── README.md                 # 이 파일
```

## 📝 각 미션 개요

### Mission 1: Unbuffered Channel 기초
- 채널 생성 및 기본 송수신
- goroutine 간 동기화
- **핵심**: 발신자와 수신자가 만나야 통신

### Mission 2: Buffered Channel
- 제한된 용량 채널
- 비동기적 송수신
- cap()/len() 함수 활용

### Mission 3: Select Statement
- 다중 채널 대기
- 비차단 연산 (default)
- 타임아웃 처리

### Mission 4: Channel Closure
- 채널 닫기 및 종료 감지
- range로 수신
- ok 패턴으로 채널 상태 확인

### Mission 5: Pipeline Pattern
- 단계별 데이터 처리
- 체이닝된 채널
- 에러 전파

### Mission 6: Fan-out/Fan-in
- 작업 분산 (병렬 처리)
- 결과 병합
- 동적 worker 관리

### Mission 7: Timeout & Context
- 타임아웃 처리
- graceful shutdown
- 컨텍스트 전파

### Mission 8: Rate Limiting
- 토큰 버킷 알고리즘
- 처리량 제어
- 동시 요청 제한

## 🛠️ 유용한 명령어

```bash
# 특정 미션의 특정 테스트만 실행
go test ./internal/mission_03 -v -run TestMission3_MultiSelect

# 커버리지 포함 테스트
go test ./... -coverprofile=coverage.out
go tool cover -html=coverage.out -o coverage.html

# 벤치마크 실행
go test ./... -bench=.

# 전체 테스트 verbose 모드
go test ./... -v
```

## 📚 참고 자료

- [Go by Example - Channels](https://gobyexample.com/channels)
- [Go Blog - Pipelines](https://go.dev/blog/pipelines)
- [Effective Go - Channels](https://go.dev/doc/effective_go#channels)
- [Go Concurrency Patterns - Google I/O 2012](https://www.youtube.com/watch?v=f6kdp27TYZs)

## 🤝 참여자

- **제자**: Roach (5년 경력 프로그래머)
- **마스터**: Sisyphus (코딩 도장 마스터)

## 📄 라이선스

MIT License

---

**계속해서 마스터의 길을 걷어가시라! 🎯**
