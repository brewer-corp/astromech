//! Shared code for the S7/S9 spike tool and its voice worker: the worker protocol,
//! the model catalogue and downloader, sample phrases, scoring and statistics.
//! Everything here is platform-neutral so it can be unit-tested on Linux.

pub mod catalog;
pub mod download;
pub mod phrases;
pub mod proto;
pub mod score;
pub mod stats;
pub mod wav;
