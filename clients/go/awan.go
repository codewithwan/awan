// Package awan drives the `awan` terminal character from Go. It shells out to
// the binary — the same personality layer every language reaches over the
// process boundary — so there is nothing to link. Install the binary with
// `npm i -g awan`, `cargo install awan-cli`, or from the GitHub Releases page.
//
// See docs/INTEGRATE.md for the full event vocabulary.
package awan

import "os/exec"

// bin is resolved from PATH; override it if you ship the binary elsewhere.
var bin = "awan"

func charArgs(character string) []string {
	if character != "" {
		return []string{"-c", character}
	}
	return nil
}

// React plays the one-shot reaction to event (e.g. "task.done"), then returns.
func React(event, character string) error {
	args := append([]string{"react", event}, charArgs(character)...)
	c := exec.Command(bin, args...)
	c.Stdout, c.Stderr = nil, nil
	return c.Run()
}

// Busy starts the "working…" loop with label. Call Cmd.Process.Kill when done.
func Busy(label, character string) (*exec.Cmd, error) {
	args := append([]string{"busy", label}, charArgs(character)...)
	c := exec.Command(bin, args...)
	return c, c.Start()
}

// Watch is an ambient companion you feed events to over time.
type Watch struct {
	cmd *exec.Cmd
	in  interface{ Write([]byte) (int, error) }
}

// StartWatch launches the companion. Emit events with (*Watch).Emit, end it
// with (*Watch).Stop.
func StartWatch(character string) (*Watch, error) {
	args := append([]string{"watch"}, charArgs(character)...)
	c := exec.Command(bin, args...)
	pipe, err := c.StdinPipe()
	if err != nil {
		return nil, err
	}
	if err := c.Start(); err != nil {
		return nil, err
	}
	return &Watch{cmd: c, in: pipe}, nil
}

// Emit pushes one event to the running companion.
func (w *Watch) Emit(event string) error {
	_, err := w.in.Write([]byte(event + "\n"))
	return err
}

// Stop ends the companion.
func (w *Watch) Stop() error { return w.cmd.Process.Kill() }
