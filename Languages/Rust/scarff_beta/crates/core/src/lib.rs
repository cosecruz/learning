//! ## **Core crate**
//! This crate contains the core business logic of **scarff** which is contained in the moduluels below;
//! - domain: pure domain models for all entities
//! - template: template specific business logic
//! - scaffold: core domain layer orchestration, orchestrates business logic

mod domain;
mod scaffold;
mod template;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
