#[derive(Debug, Clone)]
pub struct Ctx {
    pub(crate) user_id: u64,
}

// Constructor.
// design in such a way that user_id in the context cannot be changed once created
impl Ctx {
    pub fn new(user_id: u64) -> Self {
        Self { user_id }
    }
}

// Property Accessors.
impl Ctx {
    pub fn user_id(&self) -> u64 {
        self.user_id
    }
}
