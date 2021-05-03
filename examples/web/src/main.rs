#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub use gxi::*;

pub use crate::app::*;

mod app;

fn main() {
    run::<App>();
}