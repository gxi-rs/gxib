use gxi::*;

gxi! {
    App {}
    render {
        Window [
            Text ( label = "Hello World" )
        ]
    }
}

fn main() {
    run::<App>();
}
