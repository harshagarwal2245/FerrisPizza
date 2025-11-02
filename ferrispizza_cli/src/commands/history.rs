//! Show order history

use ferrispizza_lib::concurrency::SharedOrderState;

pub fn show_history(state: &SharedOrderState) {
    println!(" Order History:");

    for order in state.list_orders() {
        println!(
            " - Order #{} | {} pizzas",
            order.id.0,
            order.pizzas.len()
        );
    }
}
