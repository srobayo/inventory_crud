use std::error::Error;
use std::io;
use inventory_crud::menu;

// Helper function to avoid repeating input-reading logic
fn main() -> Result<(), Box<dyn Error>> {
  menu::run()
}
