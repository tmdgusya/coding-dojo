// Package cmd provides example executables for the Go Channel Dojo.
//
// This file demonstrates each mission's concepts in action.
// Run with: go run cmd/main.go
package main

import (
	"context"
	"fmt"
	"sync"
	"time"
)

// Mission 1: Unbuffered Channel Basics
func mission1Example() {
	fmt.Println("\n=== Mission 1: Unbuffered Channel ===")
	fmt.Println("Starting basic send/receive demo...")

	ch := make(chan string)

	go func() {
		time.Sleep(100 * time.Millisecond)
		ch <- "Hello from goroutine!"
	}()

	msg := <-ch
	fmt.Println("Received:", msg)
	fmt.Println("Mission 1 demo complete!")
}

// Mission 2: Buffered Channel
func mission2Example() {
	fmt.Println("\n=== Mission 2: Buffered Channel ===")

	ch := make(chan int, 3)
	fmt.Printf("Channel capacity: %d, length: %d\n", cap(ch), len(ch))

	ch <- 1
	ch <- 2
	ch <- 3
	fmt.Printf("After 3 sends - capacity: %d, length: %d\n", cap(ch), len(ch))

	fmt.Println("Receiving all values...")
	fmt.Println("Value 1:", <-ch)
	fmt.Println("Value 2:", <-ch)
	fmt.Println("Value 3:", <-ch)
	fmt.Println("Mission 2 demo complete!")
}

// Mission 3: Select Statement
func mission3Example() {
	fmt.Println("\n=== Mission 3: Select Statement ===")

	ch1 := make(chan string, 1)
	ch2 := make(chan string, 1)

	ch1 <- "from channel 1"

	select {
	case msg := <-ch1:
		fmt.Println("Received from ch1:", msg)
	case msg := <-ch2:
		fmt.Println("Received from ch2:", msg)
	default:
		fmt.Println("No message available")
	}

	// Non-blocking send
	select {
	case ch2 <- "message":
		fmt.Println("Sent to ch2 successfully")
	default:
		fmt.Println("Could not send to ch2")
	}

	fmt.Println("Mission 3 demo complete!")
}

// Mission 4: Channel Closure
func mission4Example() {
	fmt.Println("\n=== Mission 4: Channel Closure ===")

	ch := make(chan int, 5)

	go func() {
		for i := 1; i <= 5; i++ {
			ch <- i
		}
		close(ch)
		fmt.Println("Channel closed!")
	}()

	fmt.Println("Receiving values with range:")
	for v := range ch {
		fmt.Printf("  Value: %d\n", v)
	}

	// OK pattern
	ch2 := make(chan int)
	close(ch2)

	v, ok := <-ch2
	fmt.Printf("After close - value: %d, ok: %v\n", v, ok)
	fmt.Println("Mission 4 demo complete!")
}

// Mission 5: Pipeline Pattern
func mission5Example() {
	fmt.Println("\n=== Mission 5: Pipeline Pattern ===")

	// Generator stage
	generator := func(ctx context.Context) <-chan int {
		out := make(chan int)
		go func() {
			defer close(out)
			for i := 1; i <= 5; i++ {
				select {
				case out <- i:
				case <-ctx.Done():
					return
				}
			}
		}()
		return out
	}

	// Square stage
	square := func(ctx context.Context, in <-chan int) <-chan int {
		out := make(chan int)
		go func() {
			defer close(out)
			for n := range in {
				select {
				case out <- n * n:
				case <-ctx.Done():
					return
				}
			}
		}()
		return out
	}

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	fmt.Println("Pipeline: generator -> square")
	fmt.Print("Input:  1, 2, 3, 4, 5\n")
	fmt.Print("Output: ")
	for n := range square(ctx, generator(ctx)) {
		fmt.Printf("%d ", n)
	}
	fmt.Println()
	fmt.Println("Mission 5 demo complete!")
}

// Mission 6: Fan-out/Fan-in Pattern
func mission6Example() {
	fmt.Println("\n=== Mission 6: Fan-out/Fan-in ===")

	jobs := make(chan int, 10)
	results := make(chan int, 10)

	// Fan-out: multiple workers
	var wg sync.WaitGroup
	for w := 0; w < 3; w++ {
		wg.Add(1)
		go func(workerID int) {
			defer wg.Done()
			for job := range jobs {
				fmt.Printf("Worker %d processing job %d\n", workerID, job)
				time.Sleep(50 * time.Millisecond)
				results <- job * job
			}
		}(w)
	}

	// Send jobs
	go func() {
		for i := 1; i <= 6; i++ {
			jobs <- i
		}
		close(jobs)
	}()

	// Collect results
	go func() {
		wg.Wait()
		close(results)
	}()

	fmt.Println("Results:")
	for r := range results {
		fmt.Printf("  %d ", r)
	}
	fmt.Println()
	fmt.Println("Mission 6 demo complete!")
}

// Mission 7: Timeout & Context
func mission7Example() {
	fmt.Println("\n=== Mission 7: Timeout & Context ===")

	// Simulated work function
	doWork := func(ctx context.Context) (int, error) {
		select {
		case <-time.After(200 * time.Millisecond):
			return 42, nil
		case <-ctx.Done():
			return 0, ctx.Err()
		}
	}

	// With timeout
	ctx1, cancel1 := context.WithTimeout(context.Background(), 500*time.Millisecond)
	defer cancel1()

	start := time.Now()
	result, err := doWork(ctx1)
	elapsed := time.Since(start)
	fmt.Printf("With 500ms timeout: result=%d, err=%v, elapsed=%v\n", result, err, elapsed)

	// With short timeout
	ctx2, cancel2 := context.WithTimeout(context.Background(), 100*time.Millisecond)
	defer cancel2()

	start = time.Now()
	result, err = doWork(ctx2)
	elapsed = time.Since(start)
	fmt.Printf("With 100ms timeout: result=%d, err=%v, elapsed=%v\n", result, err, elapsed)

	fmt.Println("Mission 7 demo complete!")
}

// Mission 8: Rate Limiting
func mission8Example() {
	fmt.Println("\n=== Mission 8: Rate Limiting ===")

	rate := 5 // requests per second
	tokens := make(chan struct{}, rate*10)

	// Token bucket - refill at specified rate
	ticker := time.NewTicker(time.Second / time.Duration(rate))
	defer ticker.Stop()

	go func() {
		for range ticker.C {
			select {
			case tokens <- struct{}{}:
			default:
				// Bucket full, skip
			}
		}
	}()

	// Simulate requests
	fmt.Printf("Rate limit: %d requests/second\n", rate)
	fmt.Println("Sending 20 requests...")
	fmt.Println("Accepting:")

	accepted := 0
	for i := 0; i < 20; i++ {
		select {
		case <-tokens:
			accepted++
			fmt.Printf("  Request %d: ACCEPTED\n", i+1)
		default:
			fmt.Printf("  Request %d: DROPPED\n", i+1)
		}
	}
	fmt.Printf("Total accepted: %d\n", accepted)
	fmt.Println("Mission 8 demo complete!")
}

func main() {
	fmt.Println("╔════════════════════════════════════════════════╗")
	fmt.Println("║    Go Channel 마스터리 도장 - 예제 프로그램     ║")
	fmt.Println("╚════════════════════════════════════════════════╝")

	// Run all mission examples
	mission1Example()
	mission2Example()
	mission3Example()
	mission4Example()
	mission5Example()
	mission6Example()
	mission7Example()
	mission8Example()

	fmt.Println("\n╔════════════════════════════════════════════════╗")
	fmt.Println("║              모든 예제 완료! 🎉                 ║")
	fmt.Println("╚════════════════════════════════════════════════╝")
}
