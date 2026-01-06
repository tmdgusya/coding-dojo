// Package mission01 provides exercises for unbuffered channel basics.
//
// This package teaches:
// - Creating unbuffered channels with make(chan Type)
// - Synchronous send and receive operations
// - Goroutine synchronization via channels
package mission01

import (
	"context"
	"time"
)

// BasicSendReceive demonstrates basic channel send and receive.
// Creates an unbuffered channel and performs a synchronous operation
// between a goroutine and the main function.
//
// Returns the received message on success.
func BasicSendReceive() (string, error) {
	// TODO: Implement this function
	// 1. Create an unbuffered channel for strings
	channel := make(chan string)
	// 2. Start a goroutine that sends "Hello from goroutine!" after a delay
	go func() {
		channel <- "Hello from goroutine!"
	}()
	// 3. Receive the message in the main function
	msg := <-channel
	// 4. Return the received message
	return msg, nil
}

// SynchronizedCounter demonstrates goroutine synchronization.
// A goroutine increments a counter and signals completion via channel.
//
// Returns the final counter value.
func SynchronizedCounter() (int, error) {
	// TODO: Implement this function
	// 1. Create an unbuffered channel for signaling
	channel := make(chan int)
	// 2. Create a counter variable (int)
	counter := 41
	// 3. Start a goroutine that increments counter and sends on channel
	go func() {
		counter++
		channel <- counter
	}()
	// 4. Receive from channel to wait for completion
	<-channel
	// 5. Return the counter value
	return counter, nil
}

// PingPong demonstrates bidirectional channel communication.
// Two goroutines exchange messages via a shared channel.
//
// Returns the number of successful exchanges.
func PingPong(ctx context.Context) (int, error) {
	// TODO: Implement this function
	// 1. Create an unbuffered channel
	channel := make(chan string)
	successCount := 0
	// 2. "ping" 전송과 수신을 번갈아 수행하는 고루틴 시작
	go func() {
		for {
			select {
			case <-ctx.Done():
				return
			case channel <- "ping":
				time.Sleep(time.Millisecond * 100)
			case <-channel:
				time.Sleep(time.Millisecond * 100)
				successCount++
			}
		}
	}()
	// 3. In main, receive "ping" and send "pong" in response
	// 4. Continue for a limited number of exchanges or until context is done
	for i := 0; i < 10; i++ {
		select {
		case <-ctx.Done():
			return successCount, ctx.Err()
		case <-channel:
			channel <- "pong"
		}
	}
	// 5. Return the number of successful exchanges
	return successCount, nil
}

// DelayedMessage demonstrates timing with channels.
// A goroutine sends a message after a specified delay.
//
// message: The message to send
// delay: How long to wait before sending
//
// Returns the received message.
func DelayedMessage(message string, delay time.Duration) (string, error) {
	// TODO: Implement this function
	// 1. Create an unbuffered channel for strings
	// 2. Start a goroutine that waits for 'delay' duration then sends 'message'
	// 3. Receive the message in the main function
	// 4. Return the received message
	return "", nil
}

// WaitGroupPattern demonstrates combining channels with sync.WaitGroup.
// Multiple goroutines complete work and signal via channel.
//
// Returns the number of completed tasks.
func WaitGroupPattern() (int, error) {
	// TODO: Implement this function
	// 1. Create an unbuffered channel for results
	// 2. Launch 3 goroutines, each doing a small amount of work
	// 3. Each goroutine sends its result on the channel
	// 4. Receive all results from the channel
	// 5. Return the count of completed tasks
	return 0, nil
}
