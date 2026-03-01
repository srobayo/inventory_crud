use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum StoryError {
    ProductNotFound,
    OutOfStock(String),
    InvalidPrice,
    NonStockableItem(String),
    InsufficientStocks { name: String, available: u32 },
}

impl fmt::Display for StoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StoryError::ProductNotFound => write!(f, "Product not found"),
            StoryError::OutOfStock(s) => write!(f, "Out of Stock: {}", s),
            StoryError::InvalidPrice => write!(f, "Invalid price"),
            StoryError::NonStockableItem(name) => {
                write!(f, "Item '{}' does not support stock updates", name)
            }
            StoryError::InsufficientStocks { name, available } => {
                write!(f, "InsufficientStocks: {} {}", name, available)
            }
        }
    }
}

impl Error for StoryError {}
