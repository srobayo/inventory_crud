use crate::errors::StoryError;
pub struct Product {
    pub name: String,
    pub price: f64,
    pub quantity: u32,
}

pub struct Service {
    pub description: String,
    pub price: f64,
}

pub trait Salable {
    fn name(&self) -> &str;
    fn quantity(&self) -> u32; // para leer la cantidad actual
    fn price(&self) -> f64;
    fn is_service(&self) -> bool;
    fn add_amount(&mut self, n: u32); // para incrementar el stock
    fn make_sale(&mut self, cant: u32) -> Result<(), StoryError>; // Ahora acepta 'cant'
    fn a_csv(&self) -> String;
    fn set_price(&mut self, price: f64);
}

impl Salable for Product {
    fn name(&self) -> &str {
        &self.name
    }
    fn quantity(&self) -> u32 {
        self.quantity
    }
    fn price(&self) -> f64 {
        self.price
    }
    fn is_service(&self) -> bool {
        false
    }
    fn add_amount(&mut self, n: u32) {
        self.quantity += n; // Incrementa el stock
    }
    fn make_sale(&mut self, cant: u32) -> Result<(), StoryError> {
        if self.quantity == 0 {
            Err(StoryError::OutOfStock(self.name.clone()))
        } else if self.quantity >= cant {
            self.quantity -= cant;
            Ok(())
        } else {
            // Error detallado: Solo hay X disponibles
            Err(StoryError::InsufficientStocks {
                name: self.name.clone(),
                available: self.quantity,
            })
        }
    }
    fn a_csv(&self) -> String {
        format!("PRODUCT,{},{},{}", self.name, self.quantity, self.price)
    }
    fn set_price(&mut self, new_price: f64) {
        self.price = new_price;
    }
}

impl Salable for Service {
    fn name(&self) -> &str {
        &self.description
    }
    fn quantity(&self) -> u32 {
        0
    }
    fn price(&self) -> f64 {
        self.price
    }
    fn is_service(&self) -> bool {
        true
    }
    fn add_amount(&mut self, _n: u32) {}
    fn make_sale(&mut self, _cant: u32) -> Result<(), StoryError> {
        // Los servicios siempre tienen "stock" infinito
        println!("Servicio contratado exitosamente");
        Ok(())
    }
    fn a_csv(&self) -> String {
        format!("SERVICE,{},{}", self.description, self.price)
    }
    fn set_price(&mut self, new_price: f64) {
        self.price = new_price;
    }
}
