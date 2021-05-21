use crate::*;

gxi! {
    App {}
    render {
        Body ( style = r#"background-color : #ffffff;"# ) [
            H1 ( inner_html = "hello world" ),
            H1 ( inner_html = "hello sworld" )
        ]
    }
}
/*
Head [
                Title ( inner_html = "Hello World" ),
                Link ( rel = "stylesheet", href = "https://maxcdn.bootstrapcdn.com/bootstrap/4.5.2/css/bootstrap.min.css" ),
                Meta ( name = "viewport", content = "width=device-width, initial-scale=1" ),
                Script ( r#async = true )
            ],
            A ( href = "https://webbuddy360.com" ) [
                H1 ( inner_html = "hello world" ),
            ],
            Counter,
            CatFact,
*/
