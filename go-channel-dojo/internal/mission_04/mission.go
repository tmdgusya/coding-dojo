// Package mission04 provides exercises for channel closure.
//
// This package teaches:
// - Closing channels with close()
// - Receiving from closed channels
// - Using range to receive all values
// - The ok pattern for detecting closed channels
package mission04

import "sync"

// ReceiveUntilClose demonstrates receiving until channel is closed.
// Uses range loop to receive all values.
//
// Returns all received values as a slice.
func ReceiveUntilClose() ([]int, error) {
	// 1. Create a buffered channel
	ch := make(chan int, 5)
	// 2. Send values 1, 2, 3, 4, 5
	for i := 1; i <= 5; i++ {
		ch <- i
	}
	// 3. Close the channel
	close(ch)
	// 4. Use range to receive all values
	var slice []int
	for v := range ch {
		slice = append(slice, v)
	}
	// 5. Return the slice of received values
	return slice, nil
}

// OkPattern demonstrates the ok pattern for channel state.
// Receives from a closed channel and checks the ok value.
//
// Returns the received value and whether the channel is open.
func OkPattern() (value int, ok bool, err error) {
	// 1. Create a channel
	ch := make(chan int, 1)
	// 2. Send a value
	ch <- 42
	// 3. Close the channel
	close(ch)
	// 4. Receive using v, ok := <-ch pattern (first receive gets the value)
	value, _ = <-ch
	// 5. Second receive from closed channel returns ok=false
	_, ok = <-ch
	return value, ok, nil
}

// CloseNotify demonstrates detecting channel closure.
// Multiple receivers should all know when channel closes.
//
// Returns the number of values received before closure.
func CloseNotify() (count int, err error) {
	// 1. Create a channel
	ch := make(chan int)
	// 2. Start a goroutine that sends 3 values then closes
	go func() {
		for i := 0; i < 3; i++ {
			ch <- i
		}
		close(ch)
	}()
	// 3. Receive all values using range
	for range ch {
		count++
	}
	// 4. Return the count
	return count, nil
}

// SenderCloses demonstrates proper channel closing by sender.
// Only sender should close the channel.
//
// Returns the sum of received values.
func SenderCloses() (sum int, err error) {
	// 1. Create a channel
	ch := make(chan int)
	// 2. Start a goroutine that sends values 10, 20, 30 then closes
	go func() {
		ch <- 10
		ch <- 20
		ch <- 30
		close(ch)
	}()
	// 3. Receive all values
	for v := range ch {
		sum += v
	}
	// 4. Return the sum
	return sum, nil
}

// ConcurrencySafe demonstrates thread-safe channel operations.
// Multiple senders, one receiver.
//
// Returns the total count of received items.
func ConcurrencySafe() (count int, err error) {
	// 1. Create a channel
	ch := make(chan int, 3)
	// 2. Use sync.WaitGroup for 3 goroutines
	var wg sync.WaitGroup
	wg.Add(3)
	// 3. Each goroutine sends a different value
	for i := 1; i <= 3; i++ {
		go func(val int) {
			defer wg.Done()
			ch <- val
		}(i)
	}
	// 4. One goroutine closes after all sends complete
	go func() {
		wg.Wait()
		close(ch)
	}()
	// 5. Receive all values
	for range ch {
		count++
	}
	// 6. Return the count
	return count, nil
}
