package mission06

import (
	"sync"
	"testing"
	"time"
)

func TestMission6_FanOut(t *testing.T) {
	results, err := FanOut(6)
	if err != nil {
		t.Errorf("FanOut returned error: %v", err)
	}
	if len(results) != 6 {
		t.Errorf("FanOut() returned %d results, want 6", len(results))
	}
	for i, v := range results {
		if v != i+1 {
			t.Errorf("FanOut()[%d] = %d, want %d", i, v, i+1)
		}
	}
}

func TestMission6_FanIn(t *testing.T) {
	channels := make([]<-chan int, 3)
	for i := 0; i < 3; i++ {
		ch := make(chan int, 3)
		for j := 0; j < 3; j++ {
			ch <- i*3 + j + 1
		}
		close(ch)
		channels[i] = ch
	}

	out := FanIn(channels)
	var results []int
	for v := range out {
		results = append(results, v)
	}

	if len(results) != 9 {
		t.Errorf("FanIn() returned %d results, want 9", len(results))
	}
}

func TestMission6_Merge(t *testing.T) {
	done := make(chan struct{})
	channels := make([]<-chan int, 2)

	ch1 := make(chan int, 2)
	ch1 <- 1
	ch1 <- 2
	close(ch1)
	channels[0] = ch1

	ch2 := make(chan int, 2)
	ch2 <- 3
	ch2 <- 4
	close(ch2)
	channels[1] = ch2

	out := Merge(done, channels)
	var results []int
	for v := range out {
		results = append(results, v)
	}

	if len(results) != 4 {
		t.Errorf("Merge() returned %d results, want 4", len(results))
	}
}

func TestMission6_WorkerPool(t *testing.T) {
	results, err := WorkerPool(10, 3)
	if err != nil {
		t.Errorf("WorkerPool returned error: %v", err)
	}
	if len(results) != 10 {
		t.Errorf("WorkerPool() returned %d results, want 10", len(results))
	}
}

func TestMission6_FanIn_Concurrent(t *testing.T) {
	channels := make([]<-chan int, 3)
	for i := 0; i < 3; i++ {
		ch := make(chan int)
		go func(c chan<- int, val int) {
			time.Sleep(10 * time.Millisecond)
			c <- val
			close(c)
		}(ch, i+1)
		channels[i] = ch
	}

	out := FanIn(channels)
	var results []int
	for v := range out {
		results = append(results, v)
	}

	if len(results) != 3 {
		t.Errorf("FanIn() returned %d results, want 3", len(results))
	}
}

func TestMission6_Concurrency(t *testing.T) {
	var mu sync.Mutex
	concurrent := 0
	maxConcurrent := 0
	done := make(chan struct{})

	ch := make(chan int, 5)
	for i := 0; i < 5; i++ {
		go func(val int) {
			mu.Lock()
			concurrent++
			if concurrent > maxConcurrent {
				maxConcurrent = concurrent
			}
			mu.Unlock()

			time.Sleep(10 * time.Millisecond)

			mu.Lock()
			concurrent--
			mu.Unlock()
		}(i)
	}

	time.Sleep(50 * time.Millisecond)
	close(done)

	if maxConcurrent > 3 {
		t.Errorf("Max concurrent goroutines = %d, want <= 3", maxConcurrent)
	}
}
