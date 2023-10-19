use std::{env, fs};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let rt = env::var("ARAL_RUNTIME").unwrap_or_else(|_| "NOOP".to_string());
    let content = format!(r##"pub fn current_runtime_name() -> &'static str {{ {:?} }} "##, rt);

    let out_file = out_dir + "/out.rs";

    fs::write(out_file, content).unwrap();
}
