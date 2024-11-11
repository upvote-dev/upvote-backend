#[cfg(test)]
mod routes;

lazy_static::lazy_static! {
   pub static ref INITIATED: std::sync::Arc<std::sync::Mutex<bool>> = std::sync::Arc::new(std::sync::Mutex::new(false));
}
