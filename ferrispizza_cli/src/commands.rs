//! Command module — parses and represents CLI commands for the Pizza system.
//!
//! This module defines `Command`, which represents all actions the user can
//! perform through the CLI interface — placing orders, listing pizzas,
//! checking payment status, etc.
//!
//! Each command carries only the required data and is later processed by the
//! CLI runner module.
//!
//! # Example
//! ```
//! use pizza_store::cli::command::{Command, PizzaSelection};
//!
//! let cmd = Command::PlaceOrder {
//!     items: vec![PizzaSelection::Margherita],
//! };
//! ```

//! Command execution layer — executes parsed CLI commands.

mod menu;
mod order;
mod history;
mod pay;

pub use menu::show_menu;
pub use order::place_order;
pub use history::show_history;
pub use pay::pay_order;


/// Represents pizzas user can select through CLI.
#[derive(Debug, Clone, PartialEq)]
pub enum PizzaSelection {
    Margherita,
    Farmhouse,
    ThinMargherita,
    CheeseBurstFarmhouse,
}

/// Commands supported by CLI.
#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    /// Show menu
    Menu,

    /// Place an order with selected pizzas
    PlaceOrder {
        items: Vec<PizzaSelection>,
    },

    /// Select payment method and pay for an order
    Pay {
        order_id: u64,
        method: PaymentMethodCommand,
    },

    /// Exit application
    Exit,
}

/// CLI-level representation of payment methods
#[derive(Debug, Clone, PartialEq)]
pub enum PaymentMethodCommand {
    UPI,
    Card,
}

impl Command {
    /// Parse CLI input (tokens) into a [`Command`].
    ///
    /// # Arguments
    /// * `tokens` - Words typed by user split by spaces.
    ///
    /// # Returns
    /// A parsed command or `None` if input is invalid.
    ///
    /// # Example
    /// ```
    /// use pizza_store::cli::command::{Command, PizzaSelection};
    ///
    /// let cmd = Command::parse(&["order", "margherita"]);
    /// assert_eq!(
    ///     cmd,
    ///     Some(Command::PlaceOrder { items: vec![PizzaSelection::Margherita] })
    /// );
    /// ```
    pub fn parse(tokens: &[&str]) -> Option<Self> {
        match tokens.get(0)? {
            &"menu" => Some(Command::Menu),
            &"exit" => Some(Command::Exit),

            &"order" => {
                let mut items = vec![];
                for t in &tokens[1..] {
                    let p = match *t {
                        "margherita" => PizzaSelection::Margherita,
                        "farmhouse" => PizzaSelection::Farmhouse,
                        "thin_margherita" => PizzaSelection::ThinMargherita,
                        "cheese_burst_farmhouse" => PizzaSelection::CheeseBurstFarmhouse,
                        _ => return None,
                    };
                    items.push(p);
                }
                Some(Command::PlaceOrder { items })
            }

            &"pay" => {
                let id = tokens.get(1)?.parse().ok()?;
                let method = match *tokens.get(2)? {
                    "upi" => PaymentMethodCommand::UPI,
                    "card" => PaymentMethodCommand::Card,
                    _ => return None,
                };
                Some(Command::Pay { order_id: id, method })
            }

            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_menu() {
        assert_eq!(Command::parse(&["menu"]), Some(Command::Menu));
    }

    #[test]
    fn test_parse_exit() {
        assert_eq!(Command::parse(&["exit"]), Some(Command::Exit));
    }

    #[test]
    fn test_parse_single_order() {
        let parsed = Command::parse(&["order", "margherita"]);
        assert_eq!(
            parsed,
            Some(Command::PlaceOrder { items: vec![PizzaSelection::Margherita] })
        );
    }

    #[test]
    fn test_parse_multi_pizza_order() {
        let parsed = Command::parse(&["order", "margherita", "farmhouse"]);
        assert_eq!(
            parsed,
            Some(Command::PlaceOrder {
                items: vec![PizzaSelection::Margherita, PizzaSelection::Farmhouse]
            })
        );
    }

    #[test]
    fn test_parse_payment_upi() {
        let parsed = Command::parse(&["pay", "1", "upi"]);
        assert_eq!(
            parsed,
            Some(Command::Pay { order_id: 1, method: PaymentMethodCommand::UPI })
        );
    }

    #[test]
    fn test_parse_invalid() {
        assert_eq!(Command::parse(&["unknown"]), None);
    }
}

