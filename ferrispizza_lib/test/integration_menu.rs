use ferrispizza_core::pizza::{
    Margherita, Farmhouse, Cheese, Olives, ThinCrust, Pizza,
};

#[test]
fn test_menu_pizzas_base_costs() {
    let m = Margherita::new();
    let f = Farmhouse::new();

    assert_eq!(m.description(), "Margherita");
    assert_eq!(m.cost(), 120.0);

    assert_eq!(f.description(), "Farmhouse");
    assert_eq!(f.cost(), 150.0);
}

#[test]
fn test_menu_toppings_and_crusts() {
    let pizza = Margherita::new();
    let pizza = Cheese::new(Box::new(pizza));       // +10
    let pizza = Olives::new(Box::new(pizza));       // +15
    let pizza = ThinCrust::new(pizza);              // +20

    assert_eq!(pizza.description(), "Margherita + Cheese + Olives, Thin Crust");
    assert_eq!(pizza.cost(), 120.0 + 10.0 + 15.0 + 20.0);
}

#[test]
fn test_multi_pizza_order_menu_style() {
    let p1 = Margherita::new();
    let p2 = ThinCrust::new(Cheese::new(Box::new(Farmhouse::new())));

    assert_eq!(p1.cost(), 120.0);
    assert_eq!(p2.cost(), 150.0 + 10.0 + 20.0); // farmhouse + cheese + thin crust
}
