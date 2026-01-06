package mission04

import (
	"sync"
	"testing"
)

func TestMission4_ReceiveUntilClose(t *testing.T) {
	values, err := ReceiveUntilClose()
	if err != nil {
		t.Errorf("ReceiveUntilClose returned error: %v", err)
	}
	expected := []int{1, 2, 3, 4, 5}
	if len(values) != len(expected) {
		t.Errorf("ReceiveUntilClose() returned %d values, want %d", len(values), len(expected))
	}
	for i, v := range expected {
		if i < len(values) && values[i] != v {
			t.Errorf("ReceiveUntilClose()[%d] = %d, want %d", i, values[i], v)
		}
	}
}

func TestMission4_OkPattern(t *testing.T) {
	value, ok, err := OkPattern()
	if err != nil {
		t.Errorf("OkPattern returned error: %v", err)
	}
	if value != 42 {
		t.Errorf("OkPattern() value = %d, want 42", value)
	}
	if ok {
		t.Errorf("OkPattern() ok = true, want false (channel closed)")
	}
}

func TestMission4_CloseNotify(t *testing.T) {
	count, err := CloseNotify()
	if err != nil {
		t.Errorf("CloseNotify returned error: %v", err)
	}
	if count != 3 {
		t.Errorf("CloseNotify() count = %d, want 3", count)
	}
}

func TestMission4_SenderCloses(t *testing.T) {
	sum, err := SenderCloses()
	if err != nil {
		t.Errorf("SenderCloses returned error: %v", err)
	}
	if sum != 60 {
		t.Errorf("SenderCloses() sum = %d, want 60 (10+20+30)", sum)
	}
}

func TestMission4_ConcurrencySafe(t *testing.T) {
	var wg sync.WaitGroup
	done := make(chan bool)

	var count int
	ch := make(chan int, 3)

	wg.Add(3)
	for i := 0; i < 3; i++ {
		go func(val int) {
			defer wg.Done()
			ch <- val
		}(i + 1)
	}

	go func() {
		wg.Wait()
		close(ch)
		done <- true
	}()

	for range ch {
		count++
	}

	<-done // Wait for channel to close

	if count != 3 {
		t.Errorf("ConcurrencySafe() count = %d, want 3", count)
	}
}
