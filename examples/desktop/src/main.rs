use gxi::*;

gxi! {
    App {}
    render {
        Window [
            Text ( set_label = "Hello World" )
        ]
    }
}

fn main() {
    run::<App>();
}
