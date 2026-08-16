# sdp-inspect

A small Go CLI that parses an SDP offer/answer (RFC 4566) and prints its media
sections, directions and negotiated codecs — handy when debugging WebRTC
negotiation.

## Build & run

```bash
go build ./...
./sdp-inspect offer.sdp
# or
cat answer.sdp | go run .
```

## Example output

```
Session: "-" (v=0)
  m0: audio sendrecv [UDP/TLS/RTP/SAVPF] mid=0
      pt 111 opus/48000/2
  m1: video sendrecv [UDP/TLS/RTP/SAVPF] mid=1
      pt 96  VP8/90000
```

MIT licensed.
