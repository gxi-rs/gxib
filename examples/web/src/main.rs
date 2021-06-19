#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod app;

pub use crate::app::*;
pub use gxi::*;

fn main() {
    run::<App>();
}
