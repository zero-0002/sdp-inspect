// Command sdp-inspect prints a human-readable summary of an SDP offer/answer:
// its media sections, directions and negotiated codecs.
//
// Usage:
//
//	sdp-inspect offer.sdp
//	cat answer.sdp | sdp-inspect
package main

import (
	"fmt"
	"io"
	"os"

	"github.com/zero-0002/sdp-inspect/internal/sdp"
)

func main() {
	if err := run(os.Args[1:]); err != nil {
		fmt.Fprintln(os.Stderr, "sdp-inspect:", err)
		os.Exit(1)
	}
}

func run(args []string) error {
	var data []byte
	var err error
	if len(args) == 0 || args[0] == "-" {
		data, err = io.ReadAll(os.Stdin)
	} else {
		data, err = os.ReadFile(args[0])
	}
	if err != nil {
		return err
	}

	s := sdp.Parse(string(data))
	fmt.Printf("Session: %q (v=%s)\n", s.Name, s.Version)
	if len(s.Media) == 0 {
		fmt.Println("  (no media sections)")
		return nil
	}
	for i, m := range s.Media {
		dir := m.Direction
		if dir == "" {
			dir = "sendrecv"
		}
		fmt.Printf("  m%d: %s %s [%s] mid=%s\n", i, m.Kind, dir, m.Proto, m.Mid)
		for _, c := range m.Codecs {
			ch := ""
			if c.Channels != "" {
				ch = "/" + c.Channels
			}
			fmt.Printf("      pt %-3s %s/%s%s\n", c.PayloadType, c.Name, c.ClockRate, ch)
		}
	}
	return nil
}
