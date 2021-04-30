use crate::cli::WebArgs;

/// The web pipeline
pub fn web_pipeline(args: WebArgs) {
    println!("building web");
    if args.serve {
        println!("serving");
    }
}