pub mod admin;
mod contact;
mod sign_in;
mod text_analyzer;
mod todos;

pub use contact::post_forms_contact;
pub use sign_in::post_forms_sign_in;
pub use text_analyzer::post_forms_text_analyzer;
pub use todos::post_forms_todos;
