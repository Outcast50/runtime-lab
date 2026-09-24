use std::{env, thread};

fn main() {
    let os = env::consts::OS;
    let arch = env::consts::ARCH;
    let cpus = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);
    let cwd = env::current_dir()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "<unavailable>".to_string());

    println!("runtime-lab");
    println!("os={os}");
    println!("arch={arch}");
    println!("logical_cpus={cpus}");
    println!("cwd={cwd}");
}
