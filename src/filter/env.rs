// Copyright 2024 FastLabs Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::borrow::Cow;

use log::LevelFilter;
use log::Metadata;

use crate::filter::FilterResult;
use crate::Filter;

const DEFAULT_FILTER_ENV: &str = "RUST_LOG";

/// A filter that respects the `RUST_LOG` environment variable.
///
/// Read more from [the `env_logger` documentation](https://docs.rs/env_logger/#enabling-logging).
#[derive(Debug)]
pub struct EnvFilter(env_filter::Filter);

impl EnvFilter {
    /// Initializes the filter builder from the environment using default variable name `RUST_LOG`.
    ///
    /// # Examples
    ///
    /// Initialize a filter using the default environment variables:
    ///
    /// ```
    /// use logforth::filter::EnvFilter;
    /// let filter = EnvFilter::from_default_env();
    /// ```
    pub fn from_default_env() -> Self {
        EnvFilter::from_env(DEFAULT_FILTER_ENV)
    }

    /// Initializes the filter builder from the environment using default variable name `RUST_LOG`.
    /// If the variable is not set, the default value will be used.
    ///
    /// # Examples
    ///
    /// Initialize a filter using the default environment variables, or fallback to the default
    /// value:
    ///
    /// ```
    /// use logforth::filter::EnvFilter;
    /// let filter = EnvFilter::from_default_env_or("info");
    /// ```
    pub fn from_default_env_or<'a, V>(default: V) -> Self
    where
        V: Into<Cow<'a, str>>,
    {
        EnvFilter::from_env_or(DEFAULT_FILTER_ENV, default)
    }

    /// Initializes the filter builder from the environment using specific variable name.
    ///
    /// # Examples
    ///
    /// Initialize a filter using the using specific variable name:
    ///
    /// ```
    /// use logforth::filter::EnvFilter;
    /// let filter = EnvFilter::from_env("MY_LOG");
    /// ```
    pub fn from_env<'a, E>(name: E) -> Self
    where
        E: Into<Cow<'a, str>>,
    {
        let name = name.into();

        let builder = EnvFilterBuilder::new();
        if let Ok(s) = std::env::var(&*name) {
            EnvFilter::new(builder.parse(&s))
        } else {
            EnvFilter::new(builder)
        }
    }

    /// Initializes the filter builder from the environment using specific variable name.
    /// If the variable is not set, the default value will be used.
    ///
    /// # Examples
    ///
    /// Initialize a filter using the using specific variable name, or fallback to the default
    /// value:
    ///
    /// ```
    /// use logforth::filter::EnvFilter;
    /// let filter = EnvFilter::from_env_or("MY_LOG", "info");
    /// ```
    pub fn from_env_or<'a, 'b, E, V>(name: E, default: V) -> Self
    where
        E: Into<Cow<'a, str>>,
        V: Into<Cow<'b, str>>,
    {
        let name = name.into();
        let default = default.into();

        let builder = EnvFilterBuilder::new();
        if let Ok(s) = std::env::var(&*name) {
            EnvFilter::new(builder.parse(&s))
        } else {
            EnvFilter::new(builder.parse(&default))
        }
    }

    /// Initializes the filter builder from the [EnvFilterBuilder].
    pub fn new(mut builder: EnvFilterBuilder) -> Self {
        EnvFilter(builder.0.build())
    }

    pub(crate) fn enabled(&self, metadata: &Metadata) -> FilterResult {
        if self.0.enabled(metadata) {
            FilterResult::Neutral
        } else {
            FilterResult::Reject
        }
    }

    pub(crate) fn matches(&self, record: &log::Record) -> FilterResult {
        if self.0.matches(record) {
            FilterResult::Neutral
        } else {
            FilterResult::Reject
        }
    }
}

impl From<EnvFilter> for Filter {
    fn from(filter: EnvFilter) -> Self {
        Filter::Env(filter)
    }
}

/// A builder for the env log filter.
///
/// It can be used to parse a set of directives from a string before building a [EnvFilter]
/// instance.
#[derive(Default, Debug)]
pub struct EnvFilterBuilder(env_filter::Builder);

impl EnvFilterBuilder {
    /// Initializes the filter builder with defaults.
    pub fn new() -> Self {
        EnvFilterBuilder(env_filter::Builder::new())
    }

    /// Initializes the filter builder from an environment.
    pub fn from_env(env: &str) -> Self {
        EnvFilterBuilder(env_filter::Builder::from_env(env))
    }

    /// Adds a directive to the filter for a specific module.
    pub fn filter_module(mut self, module: &str, level: LevelFilter) -> Self {
        self.0.filter_module(module, level);
        self
    }

    /// Adds a directive to the filter for all modules.
    pub fn filter_level(mut self, level: LevelFilter) -> Self {
        self.0.filter_level(level);
        self
    }

    /// Adds a directive to the filter.
    ///
    /// The given module (if any) will log at most the specified level provided. If no module is
    /// provided then the filter will apply to all log messages.
    pub fn filter(mut self, module: Option<&str>, level: LevelFilter) -> Self {
        self.0.filter(module, level);
        self
    }

    /// Parses the directive string, returning an error if the given directive string is invalid.
    ///
    /// See [the `env_logger` documentation](https://docs.rs/env_logger/#enabling-logging) for more details.
    pub fn try_parse(mut self, filters: &str) -> anyhow::Result<Self> {
        self.0.try_parse(filters)?;
        Ok(self)
    }

    /// Parses the directives string.
    ///
    /// See [the `env_logger` documentation](https://docs.rs/env_logger/#enabling-logging) for more details.
    pub fn parse(mut self, filters: &str) -> Self {
        self.0.parse(filters);
        self
    }
}
