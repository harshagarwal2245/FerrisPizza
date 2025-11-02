//! Adapter implementations for payment processing (UPI & Card).
//!
//! This module provides:
//! - `PaymentAdapter` trait (object-safe) used by the system.
//! - Concrete adapters: `UpiPayment` and `CardPayment`.
//!
//! UPI adapter simulates random success/failure (configurable success rate).
//! Card adapter simulates simple CVV validation (configurable).

use std::thread;
use std::time::Duration;

use rand::Rng;

use crate::billing::PaymentReceipt;
use crate::errors::BillingError;
use crate::order::order;
use crate::utils::FileLogger;

/// Object-safe adapter trait for payment systems.
///
/// Implementors should perform payment processing for the provided `Order`
/// and return a `PaymentReceipt` on success or `BillingError` on failure.
pub trait PaymentAdapter: Send + Sync {
    fn pay(&self, order: &order) -> Result<PaymentReceipt, BillingError>;
}

/// Marker traits for semantic clarity (re-exported by patterns)
pub trait UpiAdapter: PaymentAdapter {}
pub trait CardAdapter: PaymentAdapter {}

/// Concrete UPI payment simulator.
///
/// `success_rate` is a value between 0.0..=1.0 indicating probability of success.
pub struct UpiPayment {
    pub upi_id: String,
    pub success_rate: f64,
    pub logger: Option<FileLogger>,
    /// simulated network delay in milliseconds
    pub delay_ms: u64,
}

impl UpiPayment {
    /// Create a UPI payment processor with default success_rate = 0.8.
    pub fn new(upi_id: impl Into<String>) -> Self {
        Self {
            upi_id: upi_id.into(),
            success_rate: 0.8,
            logger: None,
            delay_ms: 200,
        }
    }

    /// Create with explicit success rate (for testing).
    pub fn with_success_rate(upi_id: impl Into<String>, success_rate: f64) -> Self {
        Self {
            upi_id: upi_id.into(),
            success_rate: success_rate.clamp(0.0, 1.0),
            logger: None,
            delay_ms: 200,
        }
    }

    /// Attach an optional `FileLogger` to persist transaction logs.
    pub fn with_logger(mut self, logger: FileLogger) -> Self {
        self.logger = Some(logger);
        self
    }

    /// Set simulated network delay (ms).
    pub fn with_delay(mut self, ms: u64) -> Self {
        self.delay_ms = ms;
        self
    }

    fn log(&self, msg: &str) {
        if let Some(logger) = &self.logger {
            if let Err(e) = logger.log_with_timestamp(msg) {
                eprintln!("Failed to persist UPI log: {}", e);
            }
        }
    }
}

impl PaymentAdapter for UpiPayment {
    fn pay(&self, order: &order) -> Result<PaymentReceipt, BillingError> {
        // Simulate latency
        if self.delay_ms > 0 {
            thread::sleep(Duration::from_millis(self.delay_ms));
        }

        // Randomized success/failure based on success_rate
        let mut rng = rand::thread_rng();
        let chance: f64 = rng.r#gen(); // 0.0..1.0
        let success = chance <= self.success_rate;

        // Log attempt
        let log_msg = format!(
            "UPI payment attempt: upi_id={}, order_id={:?}, chance={:.3}, success={}",
            self.upi_id,
            // Debug-print order id to avoid type assumptions
            format!("{:?}", &order.id),
            chance,
            success
        );
        self.log(&log_msg);

        if success {
            // Use billing engine to build the receipt (consistent type)
            let engine = crate::billing::BillingEngine::new();
            let receipt = engine.generate_receipt(order);
            let ok_msg = format!("UPI payment success: order_id={:?}, amount={:.2}", format!("{:?}", &order.id), receipt.total_amount);
            self.log(&ok_msg);
            Ok(receipt)
        } else {
            let err_msg = format!("UPI payment failed: order_id={:?}", format!("{:?}", &order.id));
            self.log(&err_msg);
            Err(BillingError::PaymentFailed("UPI transaction rejected".into()))
        }
    }
}

impl UpiAdapter for UpiPayment {}

/// Simple Card payment simulator.
///
/// Validation rule: cvv must be a 3-digit number (100..=999) to succeed.
/// CardPayment has a `fail_on_cvv` flag for deterministic testing if desired.
pub struct CardPayment {
    pub card_number: String,
    pub cvv: u16,
    pub fail_on_invalid_cvv: bool,
    pub logger: Option<FileLogger>,
    pub delay_ms: u64,
}

impl CardPayment {
    pub fn new(card_number: impl Into<String>, cvv: u16) -> Self {
        Self {
            card_number: card_number.into(),
            cvv,
            fail_on_invalid_cvv: true,
            logger: None,
            delay_ms: 150,
        }
    }

    pub fn with_logger(mut self, logger: FileLogger) -> Self {
        self.logger = Some(logger);
        self
    }

    fn log(&self, msg: &str) {
        if let Some(logger) = &self.logger {
            if let Err(e) = logger.log_with_timestamp(msg) {
                eprintln!("Failed to persist Card log: {}", e);
            }
        }
    }

    fn validate_cvv(&self) -> bool {
        (100..=999).contains(&self.cvv)
    }
}

impl PaymentAdapter for CardPayment {
    fn pay(&self, order: &order) -> Result<PaymentReceipt, BillingError> {
        if self.delay_ms > 0 {
            thread::sleep(Duration::from_millis(self.delay_ms));
        }

        let valid = self.validate_cvv();

        let log_msg = format!(
            "Card payment attempt: card=****{}, order_id={:?}, cvv_valid={}",
            &self.card_number.chars().rev().take(4).collect::<String>().chars().rev().collect::<String>(), // last 4 masked
            format!("{:?}", &order.id),
            valid
        );

        self.log(&log_msg);

        if valid {
            let engine = crate::billing::BillingEngine::new();
            let receipt = engine.generate_receipt(order);
            let ok_msg = format!("Card payment success: order_id={:?}, amount={:.2}", format!("{:?}", &order.id), receipt.total_amount);
            self.log(&ok_msg);
            Ok(receipt)
        } else {
            let err_msg = format!("Card payment failed (invalid CVV): order_id={:?}", format!("{:?}", &order.id));
            self.log(&err_msg);
            if self.fail_on_invalid_cvv {
                Err(BillingError::PaymentFailed("Invalid CVV".into()))
            } else {
                // If fail_on_invalid_cvv == false, we still allow success (testing mode)
                let engine = crate::billing::BillingEngine::new();
                Ok(engine.generate_receipt(order))
            }
        }
    }
}

impl CardAdapter for CardPayment {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pizza::{Margherita, Cheese};
    use crate::order::order;
    use crate::utils::FileLogger;

    // Helper: build a sample order
    fn sample_order() -> order {
        let pizzas: Vec<Box<dyn crate::pizza::Pizza>> = vec![
            Box::new(Margherita::new()),
            Box::new(Cheese::new(Box::new(Margherita::new()))),
        ];
        order::new(pizzas)
    }

    #[test]
    fn upi_adapter_succeeds_when_success_rate_is_1() {
        let order = sample_order();
        let upi = UpiPayment::with_success_rate("user@upi", 1.0);
        let result = upi.pay(&order);
        assert!(result.is_ok());
        let receipt = result.unwrap();
        assert_eq!(receipt.total_amount, 250.0); // 120 + 130
    }

    #[test]
    fn upi_adapter_fails_when_success_rate_is_0() {
        let order = sample_order();
        let upi = UpiPayment::with_success_rate("user@upi", 0.0);
        let result = upi.pay(&order);
        assert!(result.is_err());
    }

    #[test]
    fn card_adapter_succeeds_with_valid_cvv() {
        let order = sample_order();
        let card = CardPayment::new("4111222233334444", 123);
        let res = card.pay(&order);
        assert!(res.is_ok());
        assert_eq!(res.unwrap().total_amount, 250.0);
    }

    #[test]
    fn card_adapter_fails_with_invalid_cvv() {
        let order = sample_order();
        let mut card = CardPayment::new("4111222233334444", 12); // invalid CVV
        card.fail_on_invalid_cvv = true;
        let res = card.pay(&order);
        assert!(res.is_err());
    }

    #[test]
    fn card_adapter_allows_when_fail_flag_disabled() {
        let order = sample_order();
        let mut card = CardPayment::new("4111222233334444", 12);
        card.fail_on_invalid_cvv = false; // test mode — allow even invalid cvv
        let res = card.pay(&order);
        assert!(res.is_ok());
    }

    #[test]
    fn adapters_log_without_panic() {
        let order = sample_order();
        // small temporary log file
        let logger = FileLogger::new("test_payment_log.txt");
        let upi = UpiPayment::with_success_rate("u@upi", 1.0).with_logger(logger);
        let _ = upi.pay(&order); // should not panic
        let _ = std::fs::remove_file("test_payment_log.txt");
    }
}
