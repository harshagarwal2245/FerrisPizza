use pizza_lib::{
    order::Order,
    pizza::{Margherita, Cheese},
    billing::BillingEngine,
    patterns::{PaymentAdapter, UpiPayment, CardPayment},
};

/// Helper — create an order with toppings
fn sample_order() -> Order {
    let pizzas: Vec<Box<dyn pizza_lib::pizza::Pizza>> = vec![
        Box::new(Margherita::new()),
        Box::new(Cheese::new(Box::new(Margherita::new()))),
    ];
    Order::new(pizzas)
}

#[test]
fn test_integration_upi_success() {
    let order = sample_order();
    let upi = UpiPayment::with_success_rate("tester@upi", 1.0); // force success

    let receipt = upi.pay(&order).expect("UPI should succeed");
    assert!(receipt.total_amount > 0.0);
    assert_eq!(receipt.total_amount, 250.0); // 120 + 130
}

#[test]
fn test_integration_upi_failure() {
    let order = sample_order();
    let upi = UpiPayment::with_success_rate("fail@upi", 0.0); // force fail

    let result = upi.pay(&order);
    assert!(result.is_err());
}

#[test]
fn test_integration_card_success() {
    let order = sample_order();
    let card = CardPayment::new("4111222233334444", 123);

    let receipt = card.pay(&order).expect("Card should succeed");
    assert_eq!(receipt.total_amount, 250.0);
}

#[test]
fn test_integration_card_invalid_cvv() {
    let order = sample_order();
    let card = CardPayment::new("4111222233334444", 12); // invalid CVV

    let result = card.pay(&order);
    assert!(result.is_err());
}

#[test]
fn test_integration_payment_trait_object() {
    let order = sample_order();
    let adapter: Box<dyn PaymentAdapter> =
        Box::new(CardPayment::new("4111222233334444", 123));

    let receipt = adapter.pay(&order).expect("Trait object card pay");
    assert_eq!(receipt.total_amount, 250.0);
}

#[test]
fn test_integration_billing_engine_receipt() {
    let order = sample_order();
    let engine = BillingEngine::new();
    let receipt = engine.generate_receipt(&order);

    assert_eq!(receipt.total_amount, 250.0);
    assert!(receipt.order_id > 0);
    assert!(receipt.timestamp > 0);
}
