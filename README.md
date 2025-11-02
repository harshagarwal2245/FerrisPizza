#  FerrisPizza — A Rust-powered Pizza Ordering System

[![License](https://img.shields.io/badge/license-MIT-blue)](LICENSE)
![Build](https://github.com/harshagarwal2245/FerrisPizza/actions/workflows/ci.yml/badge.svg)
[![Coverage](https://img.shields.io/badge/coverage-100%25-brightgreen)]()
---
FerrisPizza is a modular Rust-based pizza ordering system featuring a billing engine, extensible pizza customization, order history, and integrated payment processing. The project demonstrates idiomatic Rust application structure along with clean architectural practices — including the Decorator pattern for flexible pizza add-ons and the Adapter pattern for plugging in different payment providers — allowing future extensions without changing core logic

##  Problem Statement

Build a modular, testable, and extensible **pizza ordering system in Rust** that simulates:
- Placing pizza orders
- Viewing order history
- Paying for orders using **payment adapters**
- CLI-based interaction  
- Clean architecture, SOLID principles & design patterns
- 

---

## ✨ Features

| Capability | Description |
|----------|-------------|
 Order Pizza | Choose from multiple pizza types & toppings (Decorator pattern)  
 Order History | View all past orders  
 Multiple Payments | UPI / Card Payment adapters (Strategy + Adapter pattern)  
 Concurrency | Shared order state with threads & channels  
 Modular Architecture | Separate CLI + Core library  
 Unit & Integration Tests | Ensures correctness  
 Clean Rust | Traits, Enums, Modules, Error Handling  

---

##  Architecture Overview

FerrisPizza follows **Clean Modular Architecture**

- `ferrispizza_lib/` → Business logic  
- `ferrispizza_cli/` → CLI UI  
- Patterns Used:
  - **Decorator** → Pizza extras
  - **Strategy/Adapter** → Payments
  - **Observer-like shared state** → Orders via channels

---

##  High-Level Architecture (Mermaid)

```mermaid
flowchart TD

UI[CLI Layer]
LIB[Core Library]
ORDER_STATE[SharedOrderState]
BILLING[BillingEngine]
PIZZA[Pizza Traits & Decorators]
PAYMENT[Payment Adapter Layer]

UI --> LIB
LIB --> ORDER_STATE
LIB --> BILLING
LIB --> PIZZA
LIB --> PAYMENT
```

---

##  Class Diagram


```mermaid
classDiagram
    class Pizza {
        <<Interface>>
        + cost() f32
        + name() String
    }

    class Margherita {
        + cost() f32
        + name() String
    }

    class Farmhouse {
        + cost() f32
        + name() String
    }

    class CheeseDecorator {
        + cost() f32
    }

    class Order {
        + id: OrderId
        + pizzas: Vec<Pizza>
        + clone()
    }

    class BillingEngine {
        + calculate_total(order) f32
        + generate_receipt(order) PaymentReceipt
    }

    class PaymentAdapter {
        <<Interface>>
        + pay(order) -> Result<PaymentReceipt>
    }

    class UpiPayment
    class CardPayment

    Pizza <|.. Margherita
    Pizza <|.. Farmhouse
    Pizza <|.. CheeseDecorator

    PaymentAdapter <|.. UpiPayment
    PaymentAdapter <|.. CardPayment

    Order --> Pizza
    BillingEngine --> Order
```

---

##  Sequence Diagram (Ordering → Payment)

```mermaid
sequenceDiagram
  participant C as CLI
  participant OS as **SharedOrderState**
  participant BE as **BillingEngine**
  participant PA as **PaymentAdapter**

  C->>OS: place_order(pizzas)
  OS->>OS: store order
  C->>OS: get_order(order_id)
  OS-->>C: return order

  C->>PA: pay(order)
  PA->>BE: calculate_total()
  BE-->>PA: amount
  PA-->>C: PaymentReceipt
  C->>C: Print success
```

---

## Getting Started

###  **Run the CLI**

```bash
cd ferrispizza_cli
cargo run
```

###  Example Session

```
=== FerrisPizza Menu ===
1) Show Menu
2) Place Order
3) View Order History
4) Pay for Order
5) Exit

Enter choice: 2
Enter pizzas: margherita farmhouse
 Order placed successfully! Order ID: 1

 Enter choice: 4
Enter: pay <order_id> <upi|card>
1 upi
 Payment successful!
Total paid: 350.0
```

---

##  Project Structure

```
ferrispizza/
 ├─ ferrispizza_lib/      # Core logic (Rust library)
 ├─ ferrispizza_cli/      # CLI runner
 └─ Cargo.toml
```

---


## License

MIT © 2025 — FerrisPizza Rust Project

---
