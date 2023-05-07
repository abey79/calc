//! This module models states (as in "state machine") between the compilation steps.
//!
//! The data associated with each state is factored in "context" structure for reusability (e.g. the
//! [`TokenStream`] context ("state data") is used in both the [`TokenizedState`] and the
//! [`ParsedState`]).

pub mod input_state;
pub mod parsed_state;
pub mod tokenized_state;

pub use input_state::*;
pub use parsed_state::*;
pub use tokenized_state::*;
