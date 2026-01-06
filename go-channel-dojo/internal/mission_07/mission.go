// Package mission07 provides exercises for timeout and context patterns.
//
// This package teaches:
// - Using time.After for timeouts
// - Context cancellation with context.Context
// - Preventing goroutine leaks
package mission07

import (
	"context"
	"time"
)

// TimeoutWithAfter demonstrates timeout using time.After.
// Returns error if operation takes too long.
//
// Returns the result message or timeout error.
func TimeoutWithAfter() (string, error) {
	// TODO: Implement this function
	// 1. Create a channel
	// 2. Start a goroutine that sends after 200ms
	// 3. Use select with time.After(100ms) for timeout
	// 4. Return message or "timeout" error
	return "", nil
}

// TimeoutWithContext demonstrates timeout using context.
//
// Returns the result message or context error.
func TimeoutWithContext() (string, error) {
	// TODO: Implement this function
	// 1. Create context with 100ms timeout
	// 2. Call doWork function
	// 3. Return result or error
	return "", nil
}

// SimulatedWork simulates a long-running operation.
//
// Returns the result after a delay.
func doWork(ctx context.Context) (int, error) {
	select {
	case <-time.After(200 * time.Millisecond):
		return 42, nil
	case <-ctx.Done():
		return 0, ctx.Err()
	}
}

// ContextCancellation demonstrates cancelling a running operation.
//
// Returns the number of iterations before cancellation.
func ContextCancellation() (int, error) {
	// TODO: Implement this function
	// 1. Create a cancellable context
	// 2. Start a goroutine that increments counter until cancelled
	// 3. Cancel after 100ms
	// 4. Return the count
	return 0, nil
}

// GracefulShutdown demonstrates graceful shutdown of goroutines.
//
// Returns true if all goroutines cleaned up properly.
func GracefulShutdown() (success bool, err error) {
	// TODO: Implement this function
	// 1. Create a done channel
	// 2. Start multiple goroutines that check done channel
	// 3. Close done channel
	// 4. Wait for all goroutines to finish
	// 5. Return success
	return false, nil
}

// GoroutineLeakPrevention demonstrates preventing goroutine leaks.
// Uses buffered channel to prevent leaks on timeout.
//
// Returns the result or empty string if timed out.
func GoroutineLeakPrevention() (result string, err error) {
	// TODO: Implement this function
	// 1. Create a BUFFERED channel (capacity 1)
	// 2. Start a goroutine that sends after 200ms
	// 3. Use select with 100ms timeout
	// 4. Return result or timeout
	return "", nil
}
