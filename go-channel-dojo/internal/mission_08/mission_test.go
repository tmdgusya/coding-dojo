package mission08

import (
	"context"
	"testing"
	"time"
)

func TestMission8_TokenBucket(t *testing.T) {
	ctx := context.Background()

	// First request should be allowed
	allowed, err := TokenBucket(ctx, 5, 10)
	if err != nil {
		t.Errorf("TokenBucket returned error: %v", err)
	}
	if !allowed {
		t.Errorf("TokenBucket() first request = false, want true")
	}
}

func TestMission8_LeakyBucket(t *testing.T) {
	ctx := context.Background()

	accepted, err := LeakyBucket(ctx, 5)
	if err != nil {
		t.Errorf("LeakyBucket returned error: %v", err)
	}
	if !accepted {
		t.Errorf("LeakyBucket() first request = false, want true")
	}
}

func TestMission8_FixedWindowRateLimiter(t *testing.T) {
	// First request should be allowed
	allowed, err := FixedWindowRateLimiter(time.Second, 5)
	if err != nil {
		t.Errorf("FixedWindowRateLimiter returned error: %v", err)
	}
	if !allowed {
		t.Errorf("FixedWindowRateLimiter() first request = false, want true")
	}
}

func TestMission8_SlidingWindowRateLimiter(t *testing.T) {
	// First request should be allowed
	allowed, err := SlidingWindowRateLimiter(time.Second, 5)
	if err != nil {
		t.Errorf("SlidingWindowRateLimiter returned error: %v", err)
	}
	if !allowed {
		t.Errorf("SlidingWindowRateLimiter() first request = false, want true")
	}
}

func TestMission8_RateLimitedWorker(t *testing.T) {
	processed, err := RateLimitedWorker(10, 5, 10)
	if err != nil {
		t.Errorf("RateLimitedWorker returned error: %v", err)
	}
	if processed != 10 {
		t.Errorf("RateLimitedWorker() processed = %d, want 10", processed)
	}
}

func TestMission8_TokenBucket_Exhausted(t *testing.T) {
	ctx := context.Background()

	// Fill the bucket
	for i := 0; i < 10; i++ {
		TokenBucket(ctx, 5, 10)
	}

	// Next request should be denied
	allowed, err := TokenBucket(ctx, 5, 10)
	if err != nil {
		t.Errorf("TokenBucket returned error: %v", err)
	}
	if allowed {
		t.Errorf("TokenBucket() when exhausted = true, want false")
	}
}
