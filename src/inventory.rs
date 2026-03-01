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

    // Create
    pub fn add_and_save(&mut self, item: Box<dyn Salable>) -> Result<(), Box<dyn Error>> {
        if item.price() <= 0.0 {
            return Err(Box::new(StoryError::InvalidPrice));
        }

        let new_name = item.name().to_string();
        let quantity_to_add = item.quantity();
        let price_to_set = item.price();
        let is_incoming_service = item.is_service();

        // Check whether an item with the same name already exists
        if let Some(existing_item) = self.list.iter_mut().find(|i| i.name() == new_name) {
            if existing_item.is_service() && is_incoming_service {
                existing_item.set_price(price_to_set);
                println!("Service '{}' already existed. Price updated.", new_name);
            } else {
                // If it exists, increase stock only
                existing_item.add_amount(quantity_to_add);
                println!("Product '{}' already existed. Quantity updated.", new_name);
            }
        } else {
            // If it does not exist, add it as new
            self.list.push(item);
            println!("New product '{}' added to inventory.", new_name);
        }
        // Overwrite the file with the updated in-memory list
        self.save_all()
    }

    // Read
    pub fn load_from_file(&mut self) -> Result<(), Box<dyn Error>> {
        self.list = load_items(INVENTORY_FILE)?;
        Ok(())
    }

    // Update/Delete persistence
    pub fn save_all(&self) -> Result<(), Box<dyn Error>> {
        save_items(INVENTORY_FILE, &self.list)?;
        Ok(())
    }

    pub fn process_sale_and_save(
        &mut self,
        name: &str,
        quantity: u32,
    ) -> Result<(), Box<dyn Error>> {
        // 1. Find the item
        let item = self
            .list
            .iter_mut()
            .find(|i| i.name() == name)
            .ok_or(StoryError::ProductNotFound)?;

        // 2. Attempt the sale
        item.make_sale(quantity)?;

        // 3. Persist changes to file
        self.save_all()?;

        println!("Sale of {} units of '{}' completed.", quantity, name);

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
        println!("Price of '{}' updated to ${:.2}.", name, new_price);
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
            "Stock for '{}' increased by {}. Current total: {}",
            name, quantity, updated_quantity
        );
        Ok(())
    }

    // Find an item by name, remove it from memory, and update the file.
    pub fn delete_and_save(&mut self, name: &str) -> Result<(), Box<dyn Error>> {
        let initial_len = self.list.len();

        // Keep only those that do NOT match the target name.
        self.list.retain(|item| item.name() != name);

        if self.list.len() < initial_len {
            println!("Item '{}' removed from memory.", name);

            // Critical step: persist so the file reflects the deletion
            self.save_all()?;
            println!("File 'inventory.txt' updated successfully.");
        } else {
            println!("Item '{}' was not found for deletion.", name);
        }

        Ok(())
    }
}
