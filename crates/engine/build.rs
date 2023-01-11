use run_script::{ScriptOptions};

fn main() {
    let opts = ScriptOptions::new();
    let args = vec![];
    let (exit_code, output, error) = run_script::run(r"bash ./compile-shaders.sh", &args, &opts).unwrap();

    if exit_code != 0 || error.len() > 0 {
        println!("Failed to compile shaders. Error output:\r\n{}ERROR:\r\n{}", output, error);
        panic!("Script exited with code {}", exit_code);
    }

    println!("Shaders compiled. Output:\r\n{}", output);
}
