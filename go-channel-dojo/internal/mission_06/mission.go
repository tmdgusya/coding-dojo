// Package mission06 provides exercises for fan-out/fan-in patterns.
//
// This package teaches:
// - Distributing work across multiple goroutines
// - Collecting results from multiple channels
// - Using sync.WaitGroup for synchronization
package mission06

import (
	"sync"
)

// FanOut demonstrates distributing work to multiple workers.
// Multiple goroutines receive from the same jobs channel.
//
// Returns the results from all workers.
func FanOut(numJobs int) ([]int, error) {
	// TODO: Implement this function
	// 1. Create jobs and results channels
	// 2. Start 3 worker goroutines that process jobs
	// 3. Send numJobs to jobs channel
	// 4. Close jobs channel when all sent
	// 5. Collect results from results channel
	// 6. Return all results
	return nil, nil
}

// FanIn demonstrates merging multiple channels into one.
// Multiple input channels are merged into a single output channel.
//
// Returns all values from all input channels.
func FanIn(channels []<-chan int) <-chan int {
	// TODO: Implement this function
	// 1. Create output channel
	// 2. Use WaitGroup to wait for all input channels
	// 3. Start a goroutine for each input channel to forward values
	// 4. Close output channel when all inputs are done
	// 5. Return output channel
	out := make(chan int)
	var wg sync.WaitGroup
	wg.Add(len(channels))
	for _, ch := range channels {
		go func(c <-chan int) {
			defer wg.Done()
			for v := range c {
				out <- v
			}
		}(ch)
	}
	go func() {
		wg.Wait()
		close(out)
	}()
	return out
}

// Merge demonstrates merging channels with done channel for cancellation.
//
// Returns a channel that receives all values or nil if cancelled.
func Merge(done <-chan struct{}, channels []<-chan int) <-chan int {
	// TODO: Implement this function
	// 1. Create output channel
	// 2. Use WaitGroup to track input channels
	// 3. Start goroutine for each input channel with cancellation check
	// 4. Close output when done or all inputs closed
	// 5. Return output channel
	out := make(chan int)
	var wg sync.WaitGroup
	wg.Add(len(channels))
	for _, ch := range channels {
		go func(c <-chan int) {
			defer wg.Done()
			for {
				select {
				case v, ok := <-c:
					if !ok {
						return
					}
					select {
					case out <- v:
					case <-done:
						return
					}
				case <-done:
					return
				}
			}
		}(ch)
	}
	go func() {
		wg.Wait()
		close(out)
	}()
	return out
}

// WorkerPool creates a fixed number of workers that process jobs.
//
// Returns the results from the worker pool.
func WorkerPool(numJobs, numWorkers int) ([]int, error) {
	// TODO: Implement this function
	// 1. Create jobs and results channels
	// 2. Start numWorkers worker goroutines
	// 3. Send numJobs to jobs channel
	// 4. Close jobs channel
	// 5. Collect results from results channel
	// 6. Return all results
	return nil, nil
}
