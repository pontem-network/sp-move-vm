// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

use alloc::string::{String, ToString};

// Trait used by the VM to log interesting data.
// Clients are responsible for the implementation of alert.
pub trait LogContext: Clone {
    // Alert is called on critical errors
    fn alert(&self);
}

// Helper `Logger` implementation that does nothing
#[derive(Clone)]
pub struct NoContextLog {
    name: String,
}

impl NoContextLog {
    pub fn new() -> Self {
        Self {
            name: "test".to_string(),
        }
    }
}

impl LogContext for NoContextLog {
    fn alert(&self) {}
}

impl Default for NoContextLog {
    fn default() -> Self {
        NoContextLog::new()
    }
}
