// Package mission03 provides exercises for select statements.
//
// This package teaches:
// - Waiting on multiple channels with select
// - Default case for non-blocking operations
// - Timeout patterns with time.After
package mission03

import (
	"errors"
)

// MultiSelect demonstrates select with multiple channels.
// Receives from whichever channel has data first.
//
// Returns the message received.
func MultiSelect() (string, error) {
	// TODO: Implement this function
	// 1. Create two channels (ch1, ch2)
	// 2. Send "from ch1" to ch1 in a goroutine after a short delay
	// 3. Use select to receive from either channel
	// 4. Return the received message
	return "", nil
}

// NonBlockingReceive demonstrates non-blocking channel receive.
// Uses default case to avoid blocking when no data is available.
//
// Returns the message and whether data was received.
func NonBlockingReceive() (message string, received bool, err error) {
	// TODO: Implement this function
	// 1. Create a channel
	// 2. Use select with default case to attempt receive
	// 3. If message received, set received=true and message
	// 4. If default case, set received=false
	return "", false, nil
}

// NonBlockingSend demonstrates non-blocking channel send.
// Uses default case to avoid blocking when buffer is full.
//
// Returns whether the send succeeded.
func NonBlockingSend() (sent bool, err error) {
	// TODO: Implement this function
	// 1. Create a buffered channel with capacity 1
	// 2. Send a value (should succeed)
	// 3. Try to send another value using select with default
	// 4. Return whether the second send succeeded
	return false, nil
}

// TimeoutSelect demonstrates timeout with select.
// Returns error if operation times out.
//
// Returns the result message or timeout error.
func TimeoutSelect() (result string, err error) {
	// TODO: Implement this function
	// 1. Create a channel
	// 2. Start a goroutine that sends after 200ms
	// 3. Use select with time.After(100ms) for timeout
	// 4. Return the message or timeout error
	return "", errors.New("timeout")
}

// PrioritySelect demonstrates select with prioritized channels.
// ch1 should be checked first, then ch2, then default.
//
// Returns the message from the first available channel.
func PrioritySelect() (string, error) {
	// TODO: Implement this function
	// 1. Create two channels (ch1, ch2)
	// 2. Send to ch2 immediately (not ch1)
	// 3. Use nested select or structured approach
	// 4. First try ch1 (will block), then ch2
	// 5. Return the received message
	return "", nil
}
