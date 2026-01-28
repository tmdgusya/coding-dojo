package mission05

import (
	"context"
	"testing"
	"time"
)

func TestMission5_Generator(t *testing.T) {
	ctx, cancel := context.WithTimeout(context.Background(), time.Second)
	defer cancel()

	ch := Generator(ctx, 5)
	var received []int
	for v := range ch {
		received = append(received, v)
	}

	expected := []int{1, 2, 3, 4, 5}
	if len(received) != len(expected) {
		t.Errorf("Generator() returned %d values, want %d", len(received), len(expected))
	}
	for i, v := range expected {
		if i < len(received) && received[i] != v {
			t.Errorf("Generator()[%d] = %d, want %d", i, received[i], v)
		}
	}
}

func TestMission5_Square(t *testing.T) {
	ctx, cancel := context.WithTimeout(context.Background(), time.Second)
	defer cancel()

	in := make(chan int, 5)
	for i := 1; i <= 5; i++ {
		in <- i
	}
	close(in)

	out := Square(ctx, in)
	var received []int
	for v := range out {
		received = append(received, v)
	}

	expected := []int{1, 4, 9, 16, 25}
	if len(received) != len(expected) {
		t.Errorf("Square() returned %d values, want %d", len(received), len(expected))
	}
	for i, v := range expected {
		if i < len(received) && received[i] != v {
			t.Errorf("Square()[%d] = %d, want %d", i, received[i], v)
		}
	}
}

func TestMission5_Double(t *testing.T) {
	ctx, cancel := context.WithTimeout(context.Background(), time.Second)
	defer cancel()

	in := make(chan int, 3)
	in <- 1
	in <- 2
	in <- 3
	close(in)

	out := Double(ctx, in)
	var received []int
	for v := range out {
		received = append(received, v)
	}

	expected := []int{2, 4, 6}
	if len(received) != len(expected) {
		t.Errorf("Double() returned %d values, want %d", len(received), len(expected))
	}
	for i, v := range expected {
		if i < len(received) && received[i] != v {
			t.Errorf("Double()[%d] = %d, want %d", i, received[i], v)
		}
	}
}

func TestMission5_Pipeline(t *testing.T) {
	ctx, cancel := context.WithTimeout(context.Background(), time.Second)
	defer cancel()

	result, err := Pipeline(ctx, 3)
	if err != nil {
		t.Errorf("Pipeline returned error: %v", err)
	}
	// 1 -> 1 -> 2, 2 -> 4 -> 8, 3 -> 9 -> 18
	expected := []int{2, 8, 18}
	if len(result) != len(expected) {
		t.Errorf("Pipeline() returned %d values, want %d", len(result), len(expected))
	}
	for i, v := range expected {
		if i < len(result) && result[i] != v {
			t.Errorf("Pipeline()[%d] = %d, want %d", i, result[i], v)
		}
	}
}

func TestMission5_Filter(t *testing.T) {
	ctx, cancel := context.WithTimeout(context.Background(), time.Second)
	defer cancel()

	in := make(chan int, 5)
	for i := 1; i <= 5; i++ {
		in <- i
	}
	close(in)

	out := Filter(ctx, in, func(n int) bool { return n%2 == 0 })
	var received []int
	for v := range out {
		received = append(received, v)
	}

	expected := []int{2, 4}
	if len(received) != len(expected) {
		t.Errorf("Filter() returned %d values, want %d", len(received), len(expected))
	}
	for i, v := range expected {
		if i < len(received) && received[i] != v {
			t.Errorf("Filter()[%d] = %d, want %d", i, received[i], v)
		}
	}
}
