//! Central module for error types used across the Pizza Billing system.

mod billing_error;
mod order_error;

/// Re-exports for external use without exposing file structure.
pub use billing_error::BillingError;
pub use order_error::OrderError;
