package mission03

import (
	"testing"
	"time"
)

func TestMission3_MultiSelect(t *testing.T) {
	msg, err := MultiSelect()
	if err != nil {
		t.Errorf("MultiSelect returned error: %v", err)
	}
	if msg != "from ch1" {
		t.Errorf("MultiSelect() = %q, want %q", msg, "from ch1")
	}
}

func TestMission3_NonBlockingReceive(t *testing.T) {
	msg, received, err := NonBlockingReceive()
	if err != nil {
		t.Errorf("NonBlockingReceive returned error: %v", err)
	}
	if received {
		t.Errorf("NonBlockingReceive() received = true, want false (no data sent)")
	}
	if msg != "" {
		t.Errorf("NonBlockingReceive() message = %q, want empty", msg)
	}
}

func TestMission3_NonBlockingSend(t *testing.T) {
	sent, err := NonBlockingSend()
	if err != nil {
		t.Errorf("NonBlockingSend returned error: %v", err)
	}
	if !sent {
		t.Errorf("NonBlockingSend() sent = false, want true")
	}
}

func TestMission3_TimeoutSelect(t *testing.T) {
	start := time.Now()
	result, err := TimeoutSelect()
	elapsed := time.Since(start)

	if err == nil {
		t.Errorf("TimeoutSelect() expected timeout error, got result %q", result)
	}
	if err.Error() != "timeout" {
		t.Errorf("TimeoutSelect() error = %q, want %q", err.Error(), "timeout")
	}
	if elapsed < 100*time.Millisecond || elapsed > 300*time.Millisecond {
		t.Errorf("TimeoutSelect() elapsed = %v, want ~100-300ms", elapsed)
	}
}

func TestMission3_PrioritySelect(t *testing.T) {
	msg, err := PrioritySelect()
	if err != nil {
		t.Errorf("PrioritySelect returned error: %v", err)
	}
	if msg != "from ch2" {
		t.Errorf("PrioritySelect() = %q, want %q", msg, "from ch2")
	}
}
