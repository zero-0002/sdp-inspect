// Package sdp provides a minimal parser for Session Description Protocol
// (RFC 4566) documents, sufficient to inspect the media sections and codecs
// exchanged during WebRTC negotiation.
package sdp

import (
	"bufio"
	"strings"
)

// Media is a single "m=" section of an SDP document.
type Media struct {
	Kind      string   // audio, video, application
	Port      string   // transport port
	Proto     string   // e.g. UDP/TLS/RTP/SAVPF
	Formats   []string // payload type numbers
	Codecs    []Codec  // resolved from rtpmap attributes
	Direction string   // sendrecv, sendonly, recvonly, inactive
	Mid       string   // a=mid value
}

// Codec is a resolved a=rtpmap entry.
type Codec struct {
	PayloadType string
	Name        string
	ClockRate   string
	Channels    string
}

// Session is a parsed SDP document.
type Session struct {
	Version string
	Origin  string
	Name    string
	Media   []Media
}

// Parse reads an SDP document into a Session. It is deliberately lenient:
// unknown lines are ignored rather than rejected.
func Parse(raw string) *Session {
	s := &Session{}
	var cur *Media
	sc := bufio.NewScanner(strings.NewReader(raw))
	for sc.Scan() {
		line := strings.TrimRight(sc.Text(), "\r")
		if len(line) < 2 || line[1] != '=' {
			continue
		}
		key, val := line[0], strings.TrimSpace(line[2:])
		switch key {
		case 'v':
			s.Version = val
		case 'o':
			s.Origin = val
		case 's':
			s.Name = val
		case 'm':
			s.Media = append(s.Media, parseMedia(val))
			cur = &s.Media[len(s.Media)-1]
		case 'a':
			if cur != nil {
				applyAttribute(cur, val)
			}
		}
	}
	return s
}

func parseMedia(val string) Media {
	f := strings.Fields(val)
	m := Media{}
	if len(f) > 0 {
		m.Kind = f[0]
	}
	if len(f) > 1 {
		m.Port = f[1]
	}
	if len(f) > 2 {
		m.Proto = f[2]
	}
	if len(f) > 3 {
		m.Formats = f[3:]
	}
	return m
}

func applyAttribute(m *Media, val string) {
	switch {
	case val == "sendrecv", val == "sendonly", val == "recvonly", val == "inactive":
		m.Direction = val
	case strings.HasPrefix(val, "mid:"):
		m.Mid = strings.TrimPrefix(val, "mid:")
	case strings.HasPrefix(val, "rtpmap:"):
		m.Codecs = append(m.Codecs, parseRtpmap(strings.TrimPrefix(val, "rtpmap:")))
	}
}

func parseRtpmap(val string) Codec {
	// "96 VP8/90000" or "111 opus/48000/2"
	c := Codec{}
	parts := strings.SplitN(val, " ", 2)
	c.PayloadType = parts[0]
	if len(parts) < 2 {
		return c
	}
	enc := strings.Split(parts[1], "/")
	if len(enc) > 0 {
		c.Name = enc[0]
	}
	if len(enc) > 1 {
		c.ClockRate = enc[1]
	}
	if len(enc) > 2 {
		c.Channels = enc[2]
	}
	return c
}
