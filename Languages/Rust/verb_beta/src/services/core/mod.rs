//! ### Core
//!
//! The `core` service contains the minimum business capabilities required
//! for the MVP and for architectural exploration.
//!
//! Supported use cases:
//!
//! - Create a verb (description only)
//! - List all verbs
//! - View a single verb
//! - Update verb state (`Captured → Active → Done`)
//! - Drop a verb (with optional reason)
//! - View the action log for a verb
//!

pub(super) mod error;
pub mod infra;
///model contains all the entities and their structure and business logic
pub mod model;
///store contains their behavior and interaction as well as domain specific persistence ,ore like usecases
pub mod store;
