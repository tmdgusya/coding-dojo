// Package mission05 provides exercises for pipeline patterns.
//
// This package teaches:
// - Chaining channels together
// - Separating stages of processing
// - Context propagation for cancellation
package mission05

import (
	"context"
)

// Generator creates a stream of values.
// This is the first stage of a pipeline.
//
// Returns a channel that emits values 1 through n.
func Generator(ctx context.Context, n int) <-chan int {
	// TODO: Implement this function
	// 1. Create an output channel
	// 2. Start a goroutine that sends 1 to n
	// 3. Check context.Done() for cancellation
	// 4. Close the channel when done
	ch := make(chan int)
	go func() {
		defer close(ch)
		for i := 1; i <= n; i++ {
			select {
			case ch <- i:
			case <-ctx.Done():
				return
			}
		}
	}()
	return ch
}

// Square transforms values by squaring them.
// This is a middle stage of a pipeline.
//
// Returns a channel with squared values.
func Square(ctx context.Context, in <-chan int) <-chan int {
	// TODO: Implement this function
	// 1. Create an output channel
	// 2. Start a goroutine that receives from in, squares, sends to out
	// 3. Check context.Done() for cancellation
	// 4. Close the channel when input is closed or context cancelled
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

// Double transforms values by doubling them.
//
// Returns a channel with doubled values.
func Double(ctx context.Context, in <-chan int) <-chan int {
	// TODO: Implement this function
	// 1. Create an output channel
	// 2. Start a goroutine that doubles and sends
	// 3. Handle context cancellation
	out := make(chan int)
	go func() {
		defer close(out)
		for n := range in {
			select {
			case out <- n * 2:
			case <-ctx.Done():
				return
			}
		}
	}()
	return out
}

// Pipeline chains multiple stages together.
// generator -> square -> double
//
// Returns all values from the final stage.
func Pipeline(ctx context.Context, n int) ([]int, error) {
	// TODO: Implement this function
	// 1. Create generator with n values
	// 2. Pipe through Square stage
	// 3. Pipe through Double stage
	// 4. Collect all values from final channel
	// 5. Return collected slice
	return nil, nil
}

// Filter removes values that don't match a predicate.
//
// Returns only values where f(value) is true.
func Filter(ctx context.Context, in <-chan int, f func(int) bool) <-chan int {
	// TODO: Implement this function
	// 1. Create an output channel
	// 2. Start a goroutine that filters values
	// 3. Only send values where f(n) is true
	// 4. Handle context cancellation and channel closure
	out := make(chan int)
	go func() {
		defer close(out)
		for n := range in {
			if f(n) {
				select {
				case out <- n:
				case <-ctx.Done():
					return
				}
			}
		}
	}()
	return out
}
