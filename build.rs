use flate2::read::GzDecoder;
use std::process::{Command, Stdio};
use std::{env, ffi::OsString, path::PathBuf, str};
use tar::Archive;

const MG5_TAR: &str = "mg5/MG5_aMC_v3.5.7.tar.gz";
const MG5_DIR: &str = "MG5_aMC_v3_5_7";
const _MG5_VER: &str = "3.5.7";
const MG5_CMD: &str = "mg5/cards/standalone_sm_ma.mg5";
const STANDALONE_DIR: &str = "standalone_sm_ma";

struct Environment {
    src_dir: PathBuf,
    mg5_tar: PathBuf,
    mg5_dir: PathBuf,
    mg5_bin: PathBuf,
    py_venv: PathBuf,
    python: PathBuf,
    pip: PathBuf,
}

fn main() -> Result<(), std::io::Error> {
    let src_dir = PathBuf::from(cargo_env("CARGO_MANIFEST_DIR"));
    let _ld_path = env::var_os("LD_LIBRARY_PATH").unwrap();

    // Set environment variables
    let env = Environment {
        src_dir: src_dir.clone(),
        mg5_tar: src_dir.join(MG5_TAR),
        mg5_dir: src_dir.join(MG5_DIR),
        mg5_bin: src_dir.join(MG5_DIR).join("bin/mg5_aMC"),
        py_venv: src_dir.join(".venv"),
        python: src_dir.join(".venv/bin/python"),
        pip: src_dir.join(".venv/bin/pip"),
    };

    // Don't run if the standalone folder already exists
    if !env.src_dir.join(STANDALONE_DIR).exists() {
        // Unpack MG5 if necessary
        unpack_mg5(&env);
        // Check the python envoirament
        check_python_version().unwrap();
        create_python_venv(&env)?;

        // Run the construction of the processes
        mg5_cmd(&env, MG5_CMD.to_string());

        // Clean any previous library
        make_clean(&env);
    }

    // Build library
    make_library(&env);

    // Bridge CPP standalone to Rust
    cxx_build::bridge("src/uux_aa.rs")
        .include("./src/")
        .opt_level(3)
        .compile("rmg5-uux_aa");

    // Bridge CPP standalone to Rust
    cxx_build::bridge("src/uux_aag.rs")
        .include("./src/")
        .opt_level(3)
        .compile("rmg5-uux_aag");

    // Bridge CPP standalone to Rust
    cxx_build::bridge("src/uux_aaddx.rs")
        .include("./src/")
        .opt_level(3)
        .compile("rmg5-uux_aaddx");

    // Bridge CPP standalone to Rust
    cxx_build::bridge("src/uux_ddx.rs")
        .include("./src/")
        .opt_level(3)
        .compile("rmg5-uux_ddx");


    // Link libraries
    println!("cargo:rerun-if-changed=./lib/libmd5_class.a");
    println!("cargo:rerun-if-changed=./lib/libmodel_sm_ma.a");

    // Libraries directory
    println!(
        "cargo:lib_dir={}",
        src_dir.join("lib").to_str().unwrap()
    );
    println!(
        "cargo:rustc-link-search=native={}",
        src_dir.join("lib").to_str().unwrap()
    );
    // Link Flags
    println!("cargo:rustc-link-lib=static=rmg5",);
    println!("cargo:rustc-link-lib=static=model_sm_ma",);

    //println!(
    //    "cargo:rustc-link-arg=-Wl,-rpath,{}",
    //    src_dir.join("lib/").to_str().unwrap()
    //);

    // Update envoiraments paths
    //println!(
    //    "cargo:rustc-env=LD_LIBRARY_PATH={}:{}",
    //    src_dir.join("lib/").to_str().unwrap(),
    //    ld_path.to_str().unwrap()
    //);

    Ok(())
}

fn cargo_env(name: &str) -> OsString {
    env::var_os(name)
        .unwrap_or_else(|| panic!("environment variable not found: {}, please use cargo", name))
}

fn check_python_version() -> Result<(), String> {
    // Check for `python3 --version`
    match std::process::Command::new("python3")
        .arg("--version")
        .output()
    {
        Ok(output) => {
            if output.status.success() {
                let version = str::from_utf8(&output.stdout)
                    .or_else(|_| str::from_utf8(&output.stderr))
                    .unwrap_or("Unknown version")
                    .trim();
                //println!("cargo::warning=Found Python version: {}", version);
                let sub_version = version
                    .split(".")
                    .nth(1)
                    .unwrap()
                    .parse::<i32>()
                    .expect("Python version not a number");
                if sub_version <= 7 {
                    let msg = format!(
                        "cargo:error=Python version must be >=3.7: found {}",
                        version
                    );
                    println!("cargo:warning={msg}");
                    Err(msg)
                } else {
                    Ok(())
                }
            } else {
                Err("Failed to run: python3 --version".to_string())
            }
        }
        Err(_) => {
            println!("cargo:warning=No Python3 installation found.");
            Err("No Python3 installation found.".to_string())
        }
    }
}

fn create_python_venv(env: &Environment) -> Result<std::process::Output, std::io::Error> {
    // Create venv anew
    std::process::Command::new("python3")
        .args(["-m", "venv", "--clear", env.py_venv.to_str().unwrap()])
        .output()?;
    // Install modules
    std::process::Command::new(env.pip.clone())
        .arg("install")
        .args(["six"]) //modules
        .output()
}

fn mg5_cmd(env: &Environment, mg5_file: String) {
    // run using python from the venv
    let mut child = Command::new(env.python.clone())
        .arg(&env.mg5_bin)
        .arg(mg5_file)
        .stdout(Stdio::inherit()) // Inherit stdout (print directly)
        .stderr(Stdio::inherit()) // Inherit stderr (print errors directly)
        .spawn()
        .expect("Failed to start command");

    // Wait for the command to complete
    let status = child.wait().expect("Failed to wait for process");

    if !status.success() {
        eprintln!("Command failed with status: {}", status);
    }
}

fn make_library(env: &Environment) {
    // run using python from the venv
    let mut child = Command::new("make")
        .args([
            "-C",
            env.src_dir.to_str().unwrap(),
            "-e",
            format!("MG5={}", env.mg5_bin.to_str().unwrap()).as_str(),
            "all",
            "lib/librmg5.a",
            "lib/libmodel_sm_ma.a",
        ])
        .stdout(Stdio::inherit()) // Inherit stdout (print directly)
        .stderr(Stdio::inherit()) // Inherit stderr (print errors directly)
        .spawn()
        .expect("Failed to start command");

    // Wait for the command to complete
    let status = child.wait().expect("Failed to wait for process");

    if !status.success() {
        eprintln!("Command failed with status: {}", status);
    }
}

fn make_clean(env: &Environment) {
    // run using python from the venv
    let mut child = Command::new("make")
        .args(["-C", env.src_dir.to_str().unwrap(), "clean"])
        .stdout(Stdio::inherit()) // Inherit stdout (print directly)
        .stderr(Stdio::inherit()) // Inherit stderr (print errors directly)
        .spawn()
        .expect("Failed to start command");

    // Wait for the command to complete
    let status = child.wait().expect("Failed to wait for process");

    if !status.success() {
        eprintln!("Command failed with status: {}", status);
    }
}

fn unpack_mg5(env: &Environment) {
    // Unpack tar.gz
    if !env.mg5_dir.exists() {
        let tar = std::fs::File::open(env.mg5_tar.clone()).unwrap();
        let dec = GzDecoder::new(tar);
        let mut a = Archive::new(dec);
        a.unpack("./").unwrap();
        // Copy model folder
        let mut child = Command::new("cp")
            .arg("-r")
            .args([env.src_dir.join("mg5/sm_ma"), env.mg5_dir.join("models/.")])
            .stdout(Stdio::inherit()) // Inherit stdout (print directly)
            .stderr(Stdio::inherit()) // Inherit stderr (print errors directly)
            .spawn()
            .expect("Failed to start command");
        // Wait for the command to complete
        let status = child.wait().expect("Failed to wait for process");
        if !status.success() {
            eprintln!("Command failed with status: {}", status);
        }
    }
}
