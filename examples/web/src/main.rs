#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod app;

pub use gxi::*;
pub use crate::app::*;

fn main() {
    run::<App>();
}
