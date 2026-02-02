mod create_verb;
mod drop_verb;
mod get_logs;
mod get_verb;
mod list_verbs;
mod update_state;

pub use create_verb::create_verb;
pub use drop_verb::drop_verb;
pub use get_logs::get_verb_logs;
pub use get_verb::get_verb;
pub use list_verbs::list_verbs;
pub use update_state::update_verb_state;

// Built-in extractors:
// Extractor            What It Extracts
// Path<T>              Path parameters
// Query<T>             Query string
// Json<T>              JSON body
// Form<T>              Form data
// State<T>             Shared application state
// Extension<T>         Request extensions
// headers::HeaderMap   HTTP headers
// String               Raw body as string
// Bytes                Raw body as bytes
