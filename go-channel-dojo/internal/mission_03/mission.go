// Package mission03 provides exercises for select statements.
//
// This package teaches:
// - Waiting on multiple channels with select
// - Default case for non-blocking operations
// - Timeout patterns with time.After
package mission03

import (
	"errors"
	"time"
)

// MultiSelect demonstrates select with multiple channels.
// Receives from whichever channel has data first.
//
// Returns the message received.
func MultiSelect() (string, error) {
	// TODO: Implement this function
	// 1. Create two channels (ch1, ch2)
	channel1 := make(chan string)
	channel2 := make(chan string)
	// 2. Send "from ch1" to ch1 in a goroutine after a short delay
	go func() {
		time.Sleep(time.Millisecond)
		channel1 <- "from ch1"
	}()
	// 3. Use select to receive from either channel
	select {
	case message := <-channel1:
		return message, nil
	case message := <-channel2:
		return message, nil
	}
}

// NonBlockingReceive demonstrates non-blocking channel receive.
// Uses default case to avoid blocking when no data is available.
//
// Returns the message and whether data was received.
func NonBlockingReceive() (message string, received bool, err error) {
	// 1. Create a channel
	ch := make(chan string)
	// 2. Use select with default case to attempt receive
	select {
	case msg := <-ch:
		return msg, true, nil
	default:
		// 4. If default case, set received=false
		return "", false, nil
	}
}

// NonBlockingSend demonstrates non-blocking channel send.
// Uses default case to avoid blocking when buffer is full.
//
// Returns whether the send succeeded.
func NonBlockingSend() (sent bool, err error) {
	// 1. Create a buffered channel with capacity 1
	ch := make(chan int, 1)
	// 2. Send a value using non-blocking select
	select {
	case ch <- 1:
		return true, nil
	default:
		return false, nil
	}
}

// TimeoutSelect demonstrates timeout with select.
// Returns error if operation times out.
//
// Returns the result message or timeout error.
func TimeoutSelect() (result string, err error) {
	// 1. Create a channel
	ch := make(chan string)
	// 2. Start a goroutine that sends after 200ms
	go func() {
		time.Sleep(200 * time.Millisecond)
		ch <- "done"
	}()
	// 3. Use select with time.After(100ms) for timeout
	select {
	case result = <-ch:
		return result, nil
	case <-time.After(100 * time.Millisecond):
		// 4. Return the message or timeout error
		return "", errors.New("timeout")
	}
}

// PrioritySelect demonstrates select with prioritized channels.
// ch1 should be checked first, then ch2, then default.
//
// Returns the message from the first available channel.
func PrioritySelect() (string, error) {
	// 1. Create two channels (ch1, ch2)
	ch1 := make(chan string)
	ch2 := make(chan string)
	// 2. Send to ch2 immediately (not ch1)
	go func() {
		ch2 <- "from ch2"
	}()
	time.Sleep(time.Millisecond)
	// 3. Use nested select or structured approach
	// 4. First try ch1 (will block), then ch2
	select {
	case msg := <-ch1:
		return msg, nil
	default:
		select {
		case msg := <-ch2:
			return msg, nil
		default:
			return "", nil
		}
	}
}
