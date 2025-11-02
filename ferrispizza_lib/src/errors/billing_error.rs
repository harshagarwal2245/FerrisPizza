//! Billing related errors for the Pizza Billing system.

use thiserror::Error;

/// Represents failures that may occur during billing operations.
///
/// This error is returned by payment processors and invoice generators.
#[derive(Debug, Error)]
pub enum BillingError {
    /// Payment processing failed.
    ///
    /// Contains a message describing why the payment failed.
    #[error("Payment failed: {0}")]
    PaymentFailed(String),

    /// Invoice generation failed.
    ///
    /// Contains a message describing the invoice generation issue.
    #[error("Invoice generation failed: {0}")]
    InvoiceError(String),

    /// The selected payment method is not supported or invalid.
    #[error("Invalid payment method selected")]
    InvalidPaymentMethod,
}
