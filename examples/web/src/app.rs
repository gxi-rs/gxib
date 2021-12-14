use gxi::{gxi, Body, StrongNodeType, Text};

pub fn app() -> StrongNodeType {
    gxi! {
        Body [
            Text ( value = "hello-world" )
        ]
    }
}
