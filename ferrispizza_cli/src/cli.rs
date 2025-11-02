use crate::{commands, printer::print_welcome, input::read_choice};
use crate::app::FerrisPizzaApp;

pub fn run_cli(app: &FerrisPizzaApp) -> Result<(), String> {
    print_welcome();

    loop {
        println!("\n=== FerrisPizza Menu ===");
        println!("1) Show Menu");
        println!("2) Place Order");
        println!("3) View Order History");
        println!("4) Pay for Order");
        println!("5) Exit");

        let choice = read_choice()?;
        match choice.as_str() {
            "1" => commands::show_menu(),

            "2" => {
                println!("Enter pizzas (space separated):");
                println!("Options: margherita farmhouse thin_margherita cheese_burst_farmhouse");

                let line = read_choice()?;
                if line.trim().is_empty() {
                    println!("No pizzas entered.");
                    continue;
                }

                let mut tokens: Vec<String> = vec!["order".into()];
                tokens.extend(line.split_whitespace().map(|s| s.to_string()));
                let token_ref: Vec<&str> = tokens.iter().map(|s| s.as_str()).collect();

                match commands::Command::parse(&token_ref) {
                    Some(commands::Command::PlaceOrder { items }) => {
                        if let Err(e) = commands::place_order(&app.order_state, items) {
                            eprintln!("Failed: {:?}", e);
                        }
                    }
                    _ => println!("nvalid pizza names"),
                }
            }

            "3" => commands::show_history(&app.order_state),

            "4" => {
                println!("Enter: <order_id> <upi|card>");
                println!("Example: pay 1 upi");

                let line = read_choice()?;
                let tokens: Vec<&str> = line.split_whitespace().collect();

                match commands::Command::parse(&tokens) {
                    Some(commands::Command::Pay { order_id, method }) => {
                        if let Err(e) = commands::pay_order(&app.order_state, order_id, method) {
                            eprintln!(" Payment failed: {:?}", e);
                        }
                    }
                    _ => println!(" Invalid payment input"),
                }
            }

            "5" => {
                println!(" Goodbye!");
                return Ok(());
            }

            _ => println!("Invalid choice. Try again."),
        }
    }
}
