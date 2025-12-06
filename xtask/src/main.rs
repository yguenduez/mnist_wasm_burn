use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

type DynError = Box<dyn std::error::Error>;

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}", e);
        std::process::exit(-1);
    }
}

fn try_main() -> Result<(), DynError> {
    let task = env::args().nth(1);
    let port: u16 = env::args()
        .nth(2)
        .and_then(|port_number| port_number.parse::<u16>().ok())
        .unwrap_or(3000);

    match task.as_deref() {
        Some("serve") => serve(port)?,
        _ => print_help(),
    }
    Ok(())
}

fn print_help() {
    eprintln!(
        "Tasks:

serve <port>           builds application and server it under localhost:port
                       If no port is given, the default port is 3000.
"
    )
}

fn serve(port: u16) -> Result<(), DynError> {
    build_all()?;
    build_wasm()?;
    serve_wasm(port)?;

    Ok(())
}

fn serve_wasm(port: u16) -> Result<(), DynError> {
    let server_bin_path = project_root().join("target/release/server");
    let mnist_path = project_root().join("burn_mnist");
    Command::new(server_bin_path)
        .args(&[port.to_string()])
        .current_dir(mnist_path)
        .status()?;

    Ok(())
}

fn build_wasm() -> Result<(), DynError> {
    let mnist_path = project_root().join("burn_mnist");
    let status = Command::new("wasm-pack")
        .current_dir(mnist_path)
        .args(&[
            "build",
            "--out-dir",
            "pkg",
            "--release",
            "--target",
            "web",
            "--no-typescript",
            "--no-default-features",
        ])
        .status()?;

    if !status.success() {
        Err("cargo build failed")?;
    }
    Ok(())
}

fn build_all() -> Result<(), DynError> {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let status = Command::new(cargo)
        .current_dir(project_root())
        .args(&["build", "--release"])
        .status()?;

    if !status.success() {
        Err("cargo build failed")?;
    }
    Ok(())
}

fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}
