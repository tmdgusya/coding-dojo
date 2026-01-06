package mission01

import (
	"context"
	"testing"
	"time"
)

func TestMission1_BasicSendReceive(t *testing.T) {
	msg, err := BasicSendReceive()
	if err != nil {
		t.Errorf("BasicSendReceive returned error: %v", err)
	}
	expected := "Hello from goroutine!"
	if msg != expected {
		t.Errorf("BasicSendReceive() = %q, want %q", msg, expected)
	}
}

func TestMission1_Synchronization(t *testing.T) {
	result, err := SynchronizedCounter()
	if err != nil {
		t.Errorf("SynchronizedCounter returned error: %v", err)
	}
	if result != 42 {
		t.Errorf("SynchronizedCounter() = %d, want 42", result)
	}
}

func TestMission1_PingPong(t *testing.T) {
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	exchanges, err := PingPong(ctx)
	if err != nil {
		t.Errorf("PingPong returned error: %v", err)
	}
	if exchanges <= 0 {
		t.Errorf("PingPong() = %d, want > 0", exchanges)
	}
}

func TestMission1_DelayedMessage(t *testing.T) {
	testMessage := "Test message"
	delay := 100 * time.Millisecond

	start := time.Now()
	msg, err := DelayedMessage(testMessage, delay)
	elapsed := time.Since(start)

	if err != nil {
		t.Errorf("DelayedMessage returned error: %v", err)
	}
	if msg != testMessage {
		t.Errorf("DelayedMessage() = %q, want %q", msg, testMessage)
	}
	if elapsed < delay {
		t.Errorf("DelayedMessage() returned in %v, want at least %v", elapsed, delay)
	}
}

func TestMission1_WaitGroupPattern(t *testing.T) {
	count, err := WaitGroupPattern()
	if err != nil {
		t.Errorf("WaitGroupPattern returned error: %v", err)
	}
	if count != 3 {
		t.Errorf("WaitGroupPattern() = %d, want 3", count)
	}
}
