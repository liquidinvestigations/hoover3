//! Database related constants.

/// The Cql limit for SELECT ... WHERE field IN ? queries.
pub const CQL_SELECT_BATCH_SIZE: usize = 100;
