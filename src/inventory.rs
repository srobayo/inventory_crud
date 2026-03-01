use crate::errors::StoryError;
use crate::models::Salable;
use crate::storage::{load_items, save_items};
use std::error::Error;

const INVENTORY_FILE: &str = "inventory.txt";

pub struct Inventory {
    pub list: Vec<Box<dyn Salable>>,
}

impl Inventory {
    pub fn new() -> Self {
        Inventory { list: vec![] }
    }

    // C
    pub fn add_and_save(&mut self, item: Box<dyn Salable>) -> Result<(), Box<dyn Error>> {
        if item.price() <= 0.0 {
            return Err(Box::new(StoryError::InvalidPrice));
        }

        let new_name = item.name().to_string();
        let quantity_to_add = item.quantity();
        let price_to_set = item.price();
        let is_incoming_service = item.is_service();

        // Buscamos si ya existe un ìtem con ese noombre
        if let Some(existing_item) = self.list.iter_mut().find(|i| i.name() == new_name) {
            if existing_item.is_service() && is_incoming_service {
                existing_item.set_price(price_to_set);
                println!("El servicio '{}' ya existía. Precio actualizado.", new_name);
            } else {
                // Sì existe! Solo incrementamos su cantidad
                existing_item.add_amount(quantity_to_add);
                println!(
                    "El producto '{}' ya existìa. Cantidad actualizada.",
                    new_name
                );
            }
        } else {
            // No existe, lo añadimos como nuevo
            self.list.push(item);
            println!("Nuevo producto '{}' añadido al inventario.", new_name);
        }
        // Sobrescribimos el archivo con la lista actualizada.as
        self.save_all()
    }

    // R
    pub fn load_from_file(&mut self) -> Result<(), Box<dyn Error>> {
        self.list = load_items(INVENTORY_FILE)?;
        Ok(())
    }

    // -U- -D-
    pub fn save_all(&self) -> Result<(), Box<dyn Error>> {
        save_items(INVENTORY_FILE, &self.list)?;
        Ok(())
    }

    pub fn process_sale_and_save(
        &mut self,
        name: &str,
        quantity: u32,
    ) -> Result<(), Box<dyn Error>> {
        // 1. Buscamso el item
        let item = self
            .list
            .iter_mut()
            .find(|i| i.name() == name)
            .ok_or(StoryError::ProductNotFound)?;

        // 2. Intentamos la venta
        item.make_sale(quantity)?;

        // 3. Persistimos el cambio en el archivo
        self.save_all()?;

        println!("Venta de {} unidades de '{}' completada. ", quantity, name);

        Ok(())
    }

    pub fn update_price_and_save(
        &mut self,
        name: &str,
        new_price: f64,
    ) -> Result<(), Box<dyn Error>> {
        if new_price <= 0.0 {
            return Err(Box::new(StoryError::InvalidPrice));
        }

        let item = self
            .list
            .iter_mut()
            .find(|i| i.name() == name)
            .ok_or(StoryError::ProductNotFound)?;

        item.set_price(new_price);
        self.save_all()?;
        println!("Precio de '{}' actualizado a ${:.2} ", name, new_price);
        Ok(())
    }

    pub fn restock_and_save(&mut self, name: &str, quantity: u32) -> Result<(), Box<dyn Error>> {
        let updated_quantity = {
            let item = self
                .list
                .iter_mut()
                .find(|i| i.name() == name)
                .ok_or(StoryError::ProductNotFound)?;

            if item.is_service() {
                return Err(Box::new(StoryError::NonStockableItem(name.to_string())));
            }

            item.add_amount(quantity);
            item.quantity()
        };

        self.save_all()?;
        println!(
            "Stock de '{}' incrementado en {}. Total actual: {}",
            name, quantity, updated_quantity
        );
        Ok(())
    }

    // Busca un ìtem por nombre, lo elimina de la lista y actualiza el archivo.
    pub fn delete_and_save(&mut self, name: &str) -> Result<(), Box<dyn Error>> {
        let initial_len = self.list.len();

        // Conservamos solo los que NO coincidan con el nombre.
        self.list.retain(|item| item.name() != name);

        if self.list.len() < initial_len {
            println!("Item '{}' eliminado de la memoria. ", name);

            // Paso crucial: Guardamos la lista actualizada para que en el archivo refleje el borrado
            self.save_all()?;
            println!("Archivo 'inventory.txt' actualizado con èxito.");
        } else {
            println!("No se encontrò el ìtem '{}' para eliminarlo ", name);
        }

        Ok(())
    }
}
