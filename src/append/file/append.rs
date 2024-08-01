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

use log::Record;

use crate::append::file::non_blocking::NonBlocking;
use crate::Append;
use crate::AppendImpl;
use crate::Layout;
use crate::LayoutImpl;

#[derive(Debug)]
pub struct RollingFileAppend {
    writer: NonBlocking,
    layout: LayoutImpl,
}

impl RollingFileAppend {
    pub fn new(writer: NonBlocking) -> Self {
        Self {
            writer,
            layout: LayoutImpl::default(),
        }
    }

    pub fn with_layout(mut self, layout: impl Into<LayoutImpl>) -> Self {
        self.layout = layout.into();
        self
    }
}

impl Append for RollingFileAppend {
    fn try_append(&self, record: &Record) -> anyhow::Result<()> {
        let mut bytes = self.layout.format_bytes(record)?;
        bytes.push(b'\n');
        self.writer.send(bytes)?;
        Ok(())
    }

    fn flush(&self) {}
}

impl From<RollingFileAppend> for AppendImpl {
    fn from(append: RollingFileAppend) -> Self {
        AppendImpl::RollingFile(append)
    }
}
