// Package mission04 provides exercises for channel closure.
//
// This package teaches:
// - Closing channels with close()
// - Receiving from closed channels
// - Using range to receive all values
// - The ok pattern for detecting closed channels
package mission04

// ReceiveUntilClose demonstrates receiving until channel is closed.
// Uses range loop to receive all values.
//
// Returns all received values as a slice.
func ReceiveUntilClose() ([]int, error) {
	// TODO: Implement this function
	// 1. Create a buffered channel
	// 2. Send values 1, 2, 3, 4, 5
	// 3. Close the channel
	// 4. Use range to receive all values
	// 5. Return the slice of received values
	return nil, nil
}

// OkPattern demonstrates the ok pattern for channel state.
// Receives from a closed channel and checks the ok value.
//
// Returns the received value and whether the channel is open.
func OkPattern() (value int, ok bool, err error) {
	// TODO: Implement this function
	// 1. Create a channel
	// 2. Send a value
	// 3. Close the channel
	// 4. Receive using v, ok := <-ch pattern
	// 5. Return value and ok
	return 0, false, nil
}

// CloseNotify demonstrates detecting channel closure.
// Multiple receivers should all know when channel closes.
//
// Returns the number of values received before closure.
func CloseNotify() (count int, err error) {
	// TODO: Implement this function
	// 1. Create a channel
	// 2. Start a goroutine that sends 3 values then closes
	// 3. Receive all values using range
	// 4. Return the count
	return 0, nil
}

// SenderCloses demonstrates proper channel closing by sender.
// Only sender should close the channel.
//
// Returns the sum of received values.
func SenderCloses() (sum int, err error) {
	// TODO: Implement this function
	// 1. Create a channel
	// 2. Start a goroutine that sends values 10, 20, 30 then closes
	// 3. Receive all values
	// 4. Return the sum
	return 0, nil
}

// ConcurrencySafe demonstrates thread-safe channel operations.
// Multiple senders, one receiver.
//
// Returns the total count of received items.
func ConcurrencySafe() (count int, err error) {
	// TODO: Implement this function
	// 1. Create a channel
	// 2. Use sync.WaitGroup for 3 goroutines
	// 3. Each goroutine sends a different value
	// 4. One goroutine closes after all sends complete
	// 5. Receive all values
	// 6. Return the count
	return 0, nil
}
