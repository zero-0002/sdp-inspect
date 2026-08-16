//! Parse WebRTC ICE candidate strings into structured data.
//!
//! An ICE candidate line looks like:
//! `candidate:842163049 1 udp 1677729535 1.2.3.4 55000 typ srflx raddr 0.0.0.0 rport 0`
//!
//! This crate turns that into a [`Candidate`] you can inspect programmatically.

use std::fmt;
use std::str::FromStr;

/// The transport protocol carried by a candidate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Transport {
    Udp,
    Tcp,
    Other(String),
}

impl FromStr for Transport {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_ascii_lowercase().as_str() {
            "udp" => Transport::Udp,
            "tcp" => Transport::Tcp,
            other => Transport::Other(other.to_string()),
        })
    }
}

/// A parsed ICE candidate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Candidate {
    pub foundation: String,
    pub component: u32,
    pub transport: Transport,
    pub priority: u64,
    pub address: String,
    pub port: u16,
    pub kind: String,
    pub related_address: Option<String>,
    pub related_port: Option<u16>,
}

/// Errors that can occur while parsing a candidate line.
#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    MissingField(&'static str),
    BadNumber(&'static str),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::MissingField(name) => write!(f, "missing field: {name}"),
            ParseError::BadNumber(name) => write!(f, "invalid number for field: {name}"),
        }
    }
}

impl std::error::Error for ParseError {}

impl FromStr for Candidate {
    type Err = ParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let line = line.trim();
        let line = line.strip_prefix("a=").unwrap_or(line);
        let line = line.strip_prefix("candidate:").unwrap_or(line);

        let mut it = line.split_whitespace();
        let foundation = it.next().ok_or(ParseError::MissingField("foundation"))?.to_string();
        let component = it
            .next()
            .ok_or(ParseError::MissingField("component"))?
            .parse()
            .map_err(|_| ParseError::BadNumber("component"))?;
        let transport = it
            .next()
            .ok_or(ParseError::MissingField("transport"))?
            .parse()
            .unwrap();
        let priority = it
            .next()
            .ok_or(ParseError::MissingField("priority"))?
            .parse()
            .map_err(|_| ParseError::BadNumber("priority"))?;
        let address = it.next().ok_or(ParseError::MissingField("address"))?.to_string();
        let port = it
            .next()
            .ok_or(ParseError::MissingField("port"))?
            .parse()
            .map_err(|_| ParseError::BadNumber("port"))?;

        // Expect the literal "typ" keyword next.
        if it.next() != Some("typ") {
            return Err(ParseError::MissingField("typ"));
        }
        let kind = it.next().ok_or(ParseError::MissingField("candidate type"))?.to_string();

        let mut related_address = None;
        let mut related_port = None;
        while let Some(key) = it.next() {
            match key {
                "raddr" => related_address = it.next().map(str::to_string),
                "rport" => {
                    related_port = it
                        .next()
                        .and_then(|v| v.parse().ok());
                }
                _ => {
                    it.next();
                }
            }
        }

        Ok(Candidate {
            foundation,
            component,
            transport,
            priority,
            address,
            port,
            kind,
            related_address,
            related_port,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_srflx_candidate() {
        let c: Candidate =
            "candidate:842163049 1 udp 1677729535 1.2.3.4 55000 typ srflx raddr 0.0.0.0 rport 0"
                .parse()
                .unwrap();
        assert_eq!(c.component, 1);
        assert_eq!(c.transport, Transport::Udp);
        assert_eq!(c.address, "1.2.3.4");
        assert_eq!(c.port, 55000);
        assert_eq!(c.kind, "srflx");
        assert_eq!(c.related_address.as_deref(), Some("0.0.0.0"));
        assert_eq!(c.related_port, Some(0));
    }

    #[test]
    fn tolerates_sdp_prefix() {
        let c: Candidate = "a=candidate:1 1 tcp 2130706431 192.168.0.2 9 typ host"
            .parse()
            .unwrap();
        assert_eq!(c.transport, Transport::Tcp);
        assert_eq!(c.kind, "host");
    }
}
