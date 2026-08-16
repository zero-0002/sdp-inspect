# ice-candidate-parser

A small Rust library + CLI that parses WebRTC ICE candidate strings
(RFC 5245 / 8839) into structured fields.

## Library

```rust
use ice_candidate_parser::Candidate;

let c: Candidate = "candidate:1 1 udp 2130706431 1.2.3.4 55000 typ host"
    .parse()
    .unwrap();
assert_eq!(c.port, 55000);
```

## CLI

```bash
cargo run -- candidates.txt
# or
echo "candidate:1 1 udp 2130706431 1.2.3.4 55000 typ host" | cargo run
```

## Test

```bash
cargo test
```

MIT licensed.
