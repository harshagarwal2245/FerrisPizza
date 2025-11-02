//! Order related errors for the Pizza Billing system.

use thiserror::Error;

/// Represents failures that may occur while creating or processing an order.
#[derive(Debug, Error)]
pub enum OrderError {
    /// No pizza was selected when attempting to place an order.
    #[error("Pizza not selected")]
    NoPizza,

    /// Customer name is missing or empty.
    #[error("Customer name missing")]
    NoCustomerName,

    /// Error occurred while interacting with the internal order queue.
    ///
    /// Usually indicates a sender/receiver channel failure.
    #[error("Order queue failure: {0}")]
    QueueError(String),
}
