mod get_endpoints;
mod get_helpers;
mod post_endpoints;

pub use get_endpoints::{health_check, fetch_all_authors, fetch_all_text_types, fetch_all_text_titles_with_authors, fetch_all_text_titles_by_author};
pub use post_endpoints::{create_author, create_text};