// Package mission02 provides exercises for buffered channels.
//
// This package teaches:
// - Creating buffered channels with make(chan Type, capacity)
// - Asynchronous send operations up to buffer capacity
// - Using cap() and len() functions
package mission02

import "time"

// BufferedSend demonstrates buffered channel behavior.
// Sends multiple values to a buffered channel without blocking.
//
// Returns the final channel length.
func BufferedSend() (int, error) {
	// TODO: Implement this function
	// 1. Create a buffered channel with capacity 3
	channel := make(chan int, 3)
	// 2. Send three values: 1, 2, 3
	for i := range 3 {
		channel <- i + 1
	}
	// 3. Return the channel length (number of buffered values)
	close(channel)
	return len(channel), nil
}

// BufferCapacity demonstrates cap() and len() functions.
// Returns channel capacity and length information.
func BufferCapacity() (capacity int, length int, err error) {
	// TODO: Implement this function
	// 1. Create a buffered channel with capacity 5
	channel := make(chan string, 5)
	// 2. Send two values: "first", "second"
	channel <- "first"
	channel <- "second"
	// 3. Return the channel capacity and current length
	close(channel)
	return cap(channel), len(channel), nil
}

// BlockWhenFull demonstrates blocking behavior when buffer is full.
// Attempts to send to a full channel (should block).
//
// Returns true if all sends succeeded without blocking.
func BlockWhenFull() (bool, error) {
	// TODO: Implement this function
	// 1. Create a buffered channel with capacity 2
	channel := make(chan string, 2)
	// 2. Send two values (should not block)
	for range 2 {
		channel <- "value"
	}
	// 3. Try to send a third value in a goroutine
	go func() {
		channel <- "third"
	}()
	// 4. After a short delay, receive one value
	time.Sleep(time.Millisecond)
	<-channel
	// 5. Check if the third value was received
	select {
	case <-channel:
		return true, nil
	default:
		return false, nil
	}
}

// PartialDrain demonstrates partial channel drainage.
// Receives some values from a partially full channel.
//
// Returns the received values as a slice.
func PartialDrain() ([]int, error) {
	// TODO: Implement this function
	// 1. Create a buffered channel with capacity 5
	channel := make(chan int, 5)
	// 2. Send values: 10, 20, 30, 40, 50
	for _, value := range []int{10, 20, 30, 40, 50} {
		channel <- value
	}
	// 3. Receive only the first 3 values
	received := make([]int, 3)
	for i := range 3 {
		received[i] = <-channel
	}
	// 4. Return the received value
	return received, nil
}

// ChannelOverflow demonstrates what happens when buffer overflows.
// The goroutine should block until a value is received.
func ChannelOverflow() (sentCount int, receivedCount int, err error) {
	// TODO: Implement this function
	// 1. Create a buffered channel with capacity 2
	channel := make(chan int, 2)
	done := make(chan int)

	// 2. Send two values in main (no block)
	for range 2 {
		channel <- 0
		sentCount++
	}
	// 3. Start a goroutine that tries to send 3 more values
	go func() {
		goroutineSent := 0
		channel <- 1
		goroutineSent++
		channel <- 2
		goroutineSent++
		channel <- 3
		goroutineSent++
		done <- goroutineSent
	}()
	// 4. After a delay, receive all 5 values
	time.Sleep(time.Millisecond)
	for range 5 {
		<-channel
		receivedCount++
	}
	// 5. Receive goroutine's sent count and return
	sentCount += <-done
	return sentCount, receivedCount, nil
}
