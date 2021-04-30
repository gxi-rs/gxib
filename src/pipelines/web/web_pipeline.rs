use crate::cli::WebArgs;

/// web pipeline using wasm
pub fn web_pipeline(args: WebArgs) {
    println!("building web");
    if args.serve {
        println!("serving");
    }
}