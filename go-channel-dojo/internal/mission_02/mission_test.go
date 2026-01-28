package mission02

import (
	"testing"
)

func TestMission2_BufferedSend(t *testing.T) {
	length, err := BufferedSend()
	if err != nil {
		t.Errorf("BufferedSend returned error: %v", err)
	}
	if length != 3 {
		t.Errorf("BufferedSend() = %d, want 3", length)
	}
}

func TestMission2_BufferCapacity(t *testing.T) {
	capacity, length, err := BufferCapacity()
	if err != nil {
		t.Errorf("BufferCapacity returned error: %v", err)
	}
	if capacity != 5 {
		t.Errorf("BufferCapacity() capacity = %d, want 5", capacity)
	}
	if length != 2 {
		t.Errorf("BufferCapacity() length = %d, want 2", length)
	}
}

func TestMission2_BlockWhenFull(t *testing.T) {
	success, err := BlockWhenFull()
	if err != nil {
		t.Errorf("BlockWhenFull returned error: %v", err)
	}
	if !success {
		t.Errorf("BlockWhenFull() = false, want true (all sends should succeed)")
	}
}

func TestMission2_PartialDrain(t *testing.T) {
	received, err := PartialDrain()
	if err != nil {
		t.Errorf("PartialDrain returned error: %v", err)
	}
	expected := []int{10, 20, 30}
	if len(received) != len(expected) {
		t.Errorf("PartialDrain() returned %d values, want %d", len(received), len(expected))
	}
	for i, v := range expected {
		if i < len(received) && received[i] != v {
			t.Errorf("PartialDrain()[%d] = %d, want %d", i, received[i], v)
		}
	}
}

func TestMission2_ChannelOverflow(t *testing.T) {
	sent, received, err := ChannelOverflow()
	if err != nil {
		t.Errorf("ChannelOverflow returned error: %v", err)
	}
	if sent != 5 {
		t.Errorf("ChannelOverflow() sent = %d, want 5", sent)
	}
	if received != 5 {
		t.Errorf("ChannelOverflow() received = %d, want 5", received)
	}
}
