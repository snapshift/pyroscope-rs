// Copyright 2021 Developers of Pyroscope.

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0>. This file may not be copied, modified, or distributed
// except according to those terms.


//! Rust integration for [Pyroscope](https://pyroscope.io).
//!
//! # Quick Start
//!
//! ## Configure Pyroscope Agent
//!
//! ```ignore
//! let mut agent =
//!     PyroscopeAgent::builder("http://localhost:4040", "fibonacci")
//!        .frequency(100)
//!     .tags(
//!      [
//!       ("TagA".to_owned(), "ValueA".to_owned()),
//!    ("TagB".to_owned(), "ValueB".to_owned()),
//!      ]
//!          .iter()
//!        .cloned()
//!      .collect(),
//!         )
//!     .build()
//!    ?;
//! ```
//!
//! ## Start/Stop profiling
//! 
//! To start profiling code and sending data.
//!
//! ```ignore
//!  agent.start()?;
//! ```
//!
//! To stop profiling code. You can restart the profiling at a later point.
//!
//! ```ignore
//!  agent.stop().await?;
//! ```

// Re-exports structs
pub use crate::pyroscope::PyroscopeAgent;
pub use error::{Result, PyroscopeError};

// Public modules
pub mod pyroscope;
pub mod error;
pub mod backends;
pub mod timer;
pub mod session;

// Private modules
mod utils;
