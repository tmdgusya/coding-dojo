# Go Channel 마스터리 도장 - 문서

## 📖 도장 소개

이 도장은 Go의 Channel을 체계적으로 학습하기 위한 TDD 기반 코딩 도장입니다.
5년 경력의 프로그래머가 기초부터 마스터 수준까지 단계별로 진행합니다.

## 🎯 학습 목표

- [ ] Mission 1: Unbuffered Channel 기초 이해
- [ ] Mission 2: Buffered Channel 활용
- [ ] Mission 3: Select Statement 마스터
- [ ] Mission 4: Channel Closure 패턴 숙달
- [ ] Mission 5: Pipeline Pattern 구현
- [ ] Mission 6: Fan-out/Fan-in Pattern 이해
- [ ] Mission 7: Timeout & Context 활용
- [ ] Mission 8: Rate Limiting 구현

## 📚 학습 자료

### 공식 문서
- [Go by Example - Channels](https://gobyexample.com/channels)
- [Go by Example - Select](https://gobyexample.com/select)
- [Go by Example - Channel Buffering](https://gobyexample.com/channel-buffering)
- [Go by Example - Non-Blocking Channel Operations](https://gobyexample.com/non-blocking-channel-operations)
- [Go by Example - Closing Channels](https://gobyexample.com/closing-channels)
- [Go by Example - Worker Pools](https://gobyexample.com/worker-pools)

### Go Blog
- [Go Concurrency Patterns: Pipelines](https://go.dev/blog/pipelines)
- [Context](https://go.dev/blog/context)
- [Goroutine Preemption](https://go.dev/blog/nonblocking-channel-operation)

### Tutorial
- [A Deep Dive into Go Channels](https://www.sohamkamani.com/golang/channels/)
- [Understanding Go Channels](https://medium.com/rungo/understanding-go-channels-71e24d90c5cc)
- [Go Channels Tutorial](https://www.tutorialspoint.com/go/go_channels.htm)

## 📝 각 미션 상세

### Mission 1: Unbuffered Channel 기초

**학습 내용:**
- 채널 생성: `make(chan Type)`
- 동기식 송수신
- goroutine 간 동기화

**핵심 개념:**
- 발신자와 수신자가 만나야 통신 가능
- goroutine 없이 채널에 보내면 deadlock

**예상 출력:**
```
=== Mission 1: Unbuffered Channel ===
Main: Starting...
Goroutine: Sending "Hello, Channel!"
Main: Received: "Hello, Channel!"
```

**테스트 파일:** `internal/mission_01/test/mission_01_test.go`

---

### Mission 2: Buffered Channel

**학습 내용:**
- 버퍼 용량 지정: `make(chan Type, capacity)`
- 비동기적 송수신
- `cap()`과 `len()` 함수

**핵심 개념:**
- 버퍼가 차기 전까지 송신은 비차단
- 버퍼가 비었을 때 수신은 차단

**예상 출력:**
```
=== Mission 2: Buffered Channel ===
Capacity: 3, Length: 0
Sending 3 messages... (no block)
Capacity: 3, Length: 3 (buffer full!)
```

---

### Mission 3: Select Statement

**학습 내용:**
- 다중 채널 대기: `select { case ... }`
- 기본 케이스: `default`
- 비차단 연산

**핵심 개념:**
- 첫 번째 준비된 케이스 실행
- 다수가 준비되면 무작위 선택
- `default`로 비차단 처리

---

### Mission 4: Channel Closure

**학습 내용:**
- 채널 닫기: `close(ch)`
- `range`로 수신
- `ok` 패턴

**핵심 개념:**
- 닫힌 채널에서 영원히 제로값 수신
- `v, ok := <-ch`로 채널 상태 확인
- 송신자만 채널 닫음

---

### Mission 5: Pipeline Pattern

**학습 내용:**
- 체이닝된 채널
- 처리 단계 분리
- 에러 전파

**핵심 개념:**
- 각 단계: 수신 → 처리 → 송신
- 출력 채널 닫기로 완료 신호
- `context.Context`로 취소

---

### Mission 6: Fan-out/Fan-in

**학습 내용:**
- 작업 분산 (병렬 처리)
- 결과 병합
- `sync.WaitGroup` 활용

**핵심 개념:**
- 다중 worker가 같은 채널에서 수신
- 작업 자동 분산
- `merge()` 함수로 결과 수집

---

### Mission 7: Timeout & Context

**학습 내용:**
- `time.After()`로 타임아웃
- `context.Context` 활용
- graceful shutdown

**핵심 개념:**
- 버퍼 채널로 goroutine leak 방지
- 컨텍스트 전파로 취소 신호 전달
- 타임아웃으로 무한 대기 방지

---

### Mission 8: Rate Limiting

**학습 내용:**
- 토큰 버킷 알고리즘
- 처리량 제어
- 동시 요청 제한

**핵심 개념:**
- 버퍼 채널을 세마포어로 사용
- 토큰 채우기 속도 제어
- Drop 전략으로 부하 제어

## 🏗️ 코드 구조

### 미션별 구조

```
internal/mission_XX/
├── src/
│   └── mission.go    # 구현 (todo 플레이스홀더)
└── test/
    └── mission_XX_test.go  # RED 테스트
```

### 구현 파일 예시

```go
package mission01

// BasicSendReceive demonstrates unbuffered channel basics
func BasicSendReceive() string {
    // TODO: Implement this function
    // 1. Create an unbuffered channel
    // 2. Send a value from a goroutine
    // 3. Receive the value in main
    // 4. Return the received message
    return ""
}
```

### 테스트 파일 예시

```go
package mission01_test

import (
    "testing"
    "time"

    "github.com/roach/go-channel-dojo/internal/mission_01"
)

func TestMission1_BasicSendReceive(t *testing.T) {
    ch := make(chan string)

    go func() {
        time.Sleep(100 * time.Millisecond)
        ch <- "ping"
    }()

    select {
    case msg := <-ch:
        if msg != "ping" {
            t.Errorf("expected 'ping', got '%s'", msg)
        }
    case <-time.After(1 * time.Second):
        t.Error("timeout waiting for message")
    }
}
```

## 🧪 테스트 실행

```bash
# 특정 미션만 테스트
go test ./internal/mission_01 -v

# 특정 테스트 함수만
go test ./internal/mission_03 -v -run TestMission3_MultiSelect

# 전체 테스트
go test ./...

# 커버리지 포함
go test ./... -coverprofile=coverage.out
go tool cover -html=coverage.out -o coverage.html
```

## 💡 학습 팁

1. **단계별 진행**: 건너뛰지 말고 순서대로 진행하세요
2. **실습 중심**: 개념만 읽지 말고 직접 코드를 작성하세요
3. **테스트 활용**: 실패하는 테스트를 통과시키는 과정에서 많이 배웁니다
4. **예제 실행**: `go run cmd/main.go`로 예제를 실행해보세요
5. **문서 참조**: 도움이 필요하면 docs/Assistance.md를 확인하세요

## ❓ 질문이 있다면

- **개념 이해**: 공식 문서나 Go Blog를 참조하세요
- **구현 힌트**: docs/Assistance.md의 힌트를 확인하세요
- **직접 질문**: 마스터에게 질문하세요

---

**마스터의 길은 멀지만, 함께하면 가능합니다! 🎯**
