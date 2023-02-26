use std::str::FromStr;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Protocol {
    V1,
    V2,
}

impl FromStr for Protocol {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "1" => Protocol::V1,
            "2" => Protocol::V2,
            _ => return Err(format!("Unsupported protocol version '{}', choose '1' or '2'", s)),
        })
    }
}

#[cfg(any(feature = "blocking-client", feature = "async-client"))]
mod impls {
    use gix::protocol::transport;

    use super::Protocol;

    impl From<Protocol> for transport::Protocol {
        fn from(v: Protocol) -> Self {
            match v {
                Protocol::V1 => transport::Protocol::V1,
                Protocol::V2 => transport::Protocol::V2,
            }
        }
    }
}

impl Default for Protocol {
    fn default() -> Self {
        // Note that it's very important this remains V2, as V1 may block forever in stateful (i.e. non-http) connections when fetching
        // as we chose not to complicate matters by counting which arguments where sent (just yet).
        Protocol::V2
    }
}

#[cfg(any(feature = "async-client", feature = "blocking-client"))]
pub use gix::protocol::transport::connect;
