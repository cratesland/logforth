// Copyright 2024 tison <wander4096@gmail.com>
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

pub use dispatch::DispatchAppend;
use log::Metadata;
use log::Record;
pub use stdio::StderrAppend;
pub use stdio::StdoutAppend;

mod dispatch;
mod stdio;

pub trait Append {
    /// Whether this append is enabled; default to `true`.
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    /// Dispatches a log record to the append target.
    fn try_append(&self, record: &Record) -> anyhow::Result<()>;

    /// Flushes any buffered records.
    fn flush(&self);
}

#[derive(Debug)]
pub enum AppendImpl {
    Dispatch(DispatchAppend),
    Stdout(StdoutAppend),
    Stderr(StderrAppend),
}

impl Append for AppendImpl {
    fn enabled(&self, metadata: &Metadata) -> bool {
        match self {
            AppendImpl::Dispatch(append) => append.enabled(metadata),
            AppendImpl::Stdout(append) => append.enabled(metadata),
            AppendImpl::Stderr(append) => append.enabled(metadata),
        }
    }

    fn try_append(&self, record: &Record) -> anyhow::Result<()> {
        match self {
            AppendImpl::Dispatch(append) => append.try_append(record),
            AppendImpl::Stdout(append) => append.try_append(record),
            AppendImpl::Stderr(append) => append.try_append(record),
        }
    }

    fn flush(&self) {
        match self {
            AppendImpl::Dispatch(append) => append.flush(),
            AppendImpl::Stdout(append) => append.flush(),
            AppendImpl::Stderr(append) => append.flush(),
        }
    }
}
