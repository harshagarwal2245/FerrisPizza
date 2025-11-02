//! Billing module — provides billing engine and receipt generation.
//!
//! This module acts as the public entry point for billing components.
//! Internally, it exposes the [`BillingEngine`] which is responsible for
//! calculating total pizza cost and generating payment receipts.
mod calculator;

pub use calculator::BillingEngine;
pub use calculator::PaymentReceipt;
