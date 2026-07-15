// Use case: keep a companion on screen while a slow task runs, then react.
// The client package hides the process boundary — you just call awan.React.
package main

import (
	"time"

	awan "github.com/codewithwan/awan/clients/go"
)

func main() {
	busy, _ := awan.Busy("crunching numbers", "") // he works while you do…
	time.Sleep(2500 * time.Millisecond)           // …something slow here
	_ = busy.Process.Kill()

	_ = awan.React("task.done", "") // 🎉  (swap for "cmd.failed" on failure)
}
