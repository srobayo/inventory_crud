use inventory_crud::inventory::Inventory;
use inventory_crud::models::{Product, Service};
use std::error::Error;
use std::io;
use std::io::Write;

// Helper function to avoid repeating input-reading logic
fn read_input() -> Result<String, Box<dyn Error>> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_string())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut my_store = Inventory::new();
    // Try loading persisted inventory at startup
    let _ = my_store.load_from_file();

    loop {
        println!("\n --- PRO INVENTORY MANAGER ---");
        println!("1. List items");
        println!("2. Add product");
        println!("3. Add service");
        println!("4. Sell item");
        println!("5. Update price");
        println!("6. Delete item");
        println!("7. Update stock");
        println!("8. Exit");
        println!("Choose an option: ");
        io::stdout().flush()?; // Ensure text is printed before reading input

        let option = read_input()?;
        match option.as_str() {
            "1" => {
                println!("\n--- CURRENT INVENTORY ---");
                for item in &my_store.list {
                    println!(
                        "-- {}: (Quantity: {}, Price: {})",
                        item.name(),
                        item.quantity(),
                        item.price()
                    );
                }
            }
            "2" => {
                println!("Product name");
                let name = read_input()?;
                println!("Initial quantity");
                let quantity: u32 = read_input()?.parse().unwrap_or(0);
                println!("Initial price");
                let price = read_input()?.parse::<f64>().unwrap_or(0.0);
                my_store.add_and_save(Box::new(Product {
                    name,
                    quantity,
                    price,
                }))?;
            }
            "3" => {
                println!("Service name");
                let description = read_input()?;
                println!("Service price");
                let price = read_input()?.parse::<f64>().unwrap_or(0.0);
                my_store.add_and_save(Box::new(Service { description, price }))?;
            }
            "4" => {
                println!("What item do you want to sell?");
                let name = read_input()?;
                println!("How many units?");
                let quantity: u32 = read_input()?.parse().unwrap_or(1);

                if let Err(e) = my_store.process_sale_and_save(&name, quantity) {
                    println!("Error: {}", e);
                }
            }
            "5" => {
                println!("Item name:");
                let name = read_input()?;
                println!("New price:");
                let price: f64 = read_input()?.parse().unwrap_or(0.0);
                my_store.update_price_and_save(&name, price)?;
            }
            "6" => {
                println!("Item name to delete:");
                let name = read_input()?;
                my_store.delete_and_save(&name)?;
            }
            "7" => {
                println!("Product name to restock:");
                let name = read_input()?;
                println!("Quantity to add:");
                let quantity: u32 = read_input()?.parse().unwrap_or(0);
                if let Err(e) = my_store.restock_and_save(&name, quantity) {
                    println!("Error: {}", e);
                }
            }
            "8" => {
                println!("Goodbye!");
                break;
            }
            _ => println!(" Invalid option, please try again."),
        }
    }

    Ok(())
}
