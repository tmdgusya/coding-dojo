// Package mission02 provides exercises for buffered channels.
//
// This package teaches:
// - Creating buffered channels with make(chan Type, capacity)
// - Asynchronous send operations up to buffer capacity
// - Using cap() and len() functions
package mission02

// BufferedSend demonstrates buffered channel behavior.
// Sends multiple values to a buffered channel without blocking.
//
// Returns the final channel length.
func BufferedSend() (int, error) {
	// TODO: Implement this function
	// 1. Create a buffered channel with capacity 3
	// 2. Send three values: 1, 2, 3
	// 3. Return the channel length (number of buffered values)
	return 0, nil
}

// BufferCapacity demonstrates cap() and len() functions.
// Returns channel capacity and length information.
func BufferCapacity() (capacity int, length int, err error) {
	// TODO: Implement this function
	// 1. Create a buffered channel with capacity 5
	// 2. Send two values: "first", "second"
	// 3. Return the channel capacity and current length
	return 0, 0, nil
}

// BlockWhenFull demonstrates blocking behavior when buffer is full.
// Attempts to send to a full channel (should block).
//
// Returns true if all sends succeeded without blocking.
func BlockWhenFull() (bool, error) {
	// TODO: Implement this function
	// 1. Create a buffered channel with capacity 2
	// 2. Send two values (should not block)
	// 3. Try to send a third value in a goroutine
	// 4. After a short delay, receive one value
	// 5. Check if the third value was received
	return false, nil
}

// PartialDrain demonstrates partial channel drainage.
// Receives some values from a partially full channel.
//
// Returns the received values as a slice.
func PartialDrain() ([]int, error) {
	// TODO: Implement this function
	// 1. Create a buffered channel with capacity 5
	// 2. Send values: 10, 20, 30, 40, 50
	// 3. Receive only the first 3 values
	// 4. Return the received values
	return nil, nil
}

// ChannelOverflow demonstrates what happens when buffer overflows.
// The goroutine should block until a value is received.
func ChannelOverflow() (sentCount int, receivedCount int, err error) {
	// TODO: Implement this function
	// 1. Create a buffered channel with capacity 2
	// 2. Send two values in main (no block)
	// 3. Start a goroutine that tries to send 3 more values
	// 4. After a delay, receive all 5 values
	// 5. Return sent count and received count
	return 0, 0, nil
}
