use inventory_crud::inventory::Inventory;
use inventory_crud::models::{Product, Service};
use std::error::Error;
use std::io;
use std::io::Write;

// Funciòn auxiliar para no repetir còdigo de lectura
fn read_input() -> Result<String, Box<dyn Error>> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_string())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut my_store = Inventory::new();
    // Intentamos cargar al iniciar
    let _ = my_store.load_from_file();

    loop {
        println!("\n --- GESTOR DE INVENTARIO PRO ---");
        println!("1. Listar productos");
        println!("2. Agregar Producto");
        println!("3. Agregar Servicio");
        println!("4. Vender ìtem");
        println!("5. Actualizar Precio");
        println!("6. Eliminar ìtem");
        println!("7. Actualizar Stock");
        println!("8. Salir");
        println!("Seleccione una opciòn: ");
        io::stdout().flush()?; // Asegura que el texto se imprima antes de pedir entrada

        let option = read_input()?;
        match option.as_str() {
            "1" => {
                println!("\n--- INVENTARIO ACTUAL ---");
                for item in &my_store.list {
                    println!(
                        "-- {}: (Cantidad: {}, Precio: {})",
                        item.name(),
                        item.quantity(),
                        item.price()
                    );
                }
            }
            "2" => {
                println!("Nombre del productos");
                let name = read_input()?;
                println!("Cantidad inicial");
                let quantity: u32 = read_input()?.parse().unwrap_or(0);
                println!("Precio inicial");
                let price = read_input()?.parse::<f64>().unwrap_or(0.0);
                my_store.add_and_save(Box::new(Product {
                    name,
                    quantity,
                    price,
                }))?;
            }
            "3" => {
                println!("Nombre del Servicio");
                let description = read_input()?;
                println!("Precio del Servicio");
                let price = read_input()?.parse::<f64>().unwrap_or(0.0);
                my_store.add_and_save(Box::new(Service { description, price }))?;
            }
            "4" => {
                println!("¿Què desea vender?");
                let name = read_input()?;
                println!("¿Cuàntas unidades?");
                let quantity: u32 = read_input()?.parse().unwrap_or(1);

                if let Err(e) = my_store.process_sale_and_save(&name, quantity) {
                    println!("Error: {}", e);
                }
            }
            "5" => {
                println!("Nombre del ìtem:");
                let name = read_input()?;
                println!("Nuevo precio:");
                let price: f64 = read_input()?.parse().unwrap_or(0.0);
                my_store.update_price_and_save(&name, price)?;
            }
            "6" => {
                println!("Nombre del ìtem a eliminar:");
                let name = read_input()?;
                my_store.delete_and_save(&name)?;
            }
            "7" => {
                println!("Nombre del producto a reponer:");
                let name = read_input()?;
                println!("Cantidad a agregar:");
                let quantity: u32 = read_input()?.parse().unwrap_or(0);
                if let Err(e) = my_store.restock_and_save(&name, quantity) {
                    println!("Error: {}", e);
                }
            }
            "8" => {
                println!("¡Hasta luego!");
                break;
            }
            _ => println!(" Opciòn no vàlida, intente de nuevo."),
        }
    }

    Ok(())
}
