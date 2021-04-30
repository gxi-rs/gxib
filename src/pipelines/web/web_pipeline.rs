use crate::cli::WebArgs;
use crate::utils::CargoToml;

/// web pipeline using wasm
pub fn web_pipeline(args: WebArgs, cargo_toml: &mut CargoToml) {
    println!("building web");
    if args.serve {
        println!("serving");
    }
}
