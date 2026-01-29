/// trait that describes the use cases of verb
pub trait Doing {
    ///create new verb
    fn create(&self, title: Option<String>, desc: String) {}
}

///struct to define verb use cases
struct VerbStore {}

impl Doing for VerbStore {
    fn create(&self, title: Option<String>, desc: String) {
        //
    }
}
