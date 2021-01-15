//! The match_string filter.
//!
//! Requires the `match_string_filter` feature.

use log::Record;

#[cfg(feature = "config_parsing")]
use crate::config::{Deserialize, Deserializers};
use crate::filter::{Filter, Response};

/// The threshold filter's configuration.
#[cfg(feature = "config_parsing")]
#[derive(Clone, Eq, PartialEq, Hash, Debug, serde::Deserialize)]
pub struct MatchStringFilterConfig {
    matched: String,
}

/// A filter that rejects all events with body that not contains a provided substring.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct MatchStringFilter {
    matched: String,
}

impl MatchStringFilter {
    /// Creates a new `MatchStringFilter` with the specified substring.
    pub fn new(matched: &str) -> MatchStringFilter {
        MatchStringFilter {
            matched: matched.to_string(),
        }
    }
}

impl Filter for MatchStringFilter {
    fn filter(&self, record: &Record) -> Response {
        if !record.args().to_string().contains(&self.matched) {
            Response::Reject
        } else {
            Response::Neutral
        }
    }
}

/// A deserializer for the `MatchStringFilter`.
///
/// # Configuration
///
/// ```yaml
/// kind: match_string
///
/// # The threshold log level to filter at. Required
/// matched: " SERVICE: "
/// ```
#[cfg(feature = "config_parsing")]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct MatchStringFilterDeserializer;

#[cfg(feature = "config_parsing")]
impl Deserialize for MatchStringFilterDeserializer {
    type Trait = dyn Filter;

    type Config = MatchStringFilterConfig;

    fn deserialize(
        &self,
        config: MatchStringFilterConfig,
        _: &Deserializers,
    ) -> anyhow::Result<Box<dyn Filter>> {
        Ok(Box::new(MatchStringFilter::new(&config.matched)))
    }
}
