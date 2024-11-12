#[cfg(test)]
pub static INIT_DB: std::sync::Once = std::sync::Once::new();

#[cfg(test)]
mod profile;

#[cfg(test)]
mod review;
