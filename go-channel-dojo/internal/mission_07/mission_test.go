package mission07

import (
	"context"
	"errors"
	"testing"
	"time"
)

func TestMission7_TimeoutWithAfter(t *testing.T) {
	start := time.Now()
	result, err := TimeoutWithAfter()
	elapsed := time.Since(start)

	if err == nil {
		t.Errorf("TimeoutWithAfter() expected error, got result %q", result)
	}
	if !errors.Is(err, context.DeadlineExceeded) {
		t.Errorf("TimeoutWithAfter() error = %v, want deadline exceeded", err)
	}
	if elapsed < 100*time.Millisecond || elapsed > 200*time.Millisecond {
		t.Errorf("TimeoutWithAfter() elapsed = %v, want ~100-200ms", elapsed)
	}
}

func TestMission7_TimeoutWithContext(t *testing.T) {
	start := time.Now()
	result, err := TimeoutWithContext()
	elapsed := time.Since(start)

	if err == nil {
		t.Errorf("TimeoutWithContext() expected error, got result %q", result)
	}
	if elapsed < 100*time.Millisecond || elapsed > 200*time.Millisecond {
		t.Errorf("TimeoutWithContext() elapsed = %v, want ~100-200ms", elapsed)
	}
}

func TestMission7_ContextCancellation(t *testing.T) {
	count, err := ContextCancellation()
	if err != nil {
		t.Errorf("ContextCancellation returned error: %v", err)
	}
	if count <= 0 {
		t.Errorf("ContextCancellation() count = %d, want > 0", count)
	}
}

func TestMission7_GracefulShutdown(t *testing.T) {
	success, err := GracefulShutdown()
	if err != nil {
		t.Errorf("GracefulShutdown returned error: %v", err)
	}
	if !success {
		t.Errorf("GracefulShutdown() success = false, want true")
	}
}

func TestMission7_GoroutineLeakPrevention(t *testing.T) {
	result, err := GoroutineLeakPrevention()
	if err != nil {
		t.Errorf("GoroutineLeakPrevention returned error: %v", err)
	}
	if result != "" {
		t.Errorf("GoroutineLeakPrevention() result = %q, want empty", result)
	}
}

func TestMission7_ContextWithTimeout(t *testing.T) {
	ctx, cancel := context.WithTimeout(context.Background(), 50*time.Millisecond)
	defer cancel()

	result, err := doWork(ctx)
	if err == nil {
		t.Errorf("doWork() expected timeout error, got %d", result)
	}
	if !errors.Is(err, context.DeadlineExceeded) {
		t.Errorf("doWork() error = %v, want deadline exceeded", err)
	}
}
