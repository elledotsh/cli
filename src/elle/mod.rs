use php_codegen::comment::Document;
use php_codegen::data_type::DataType;
use php_codegen::file::File;
use php_codegen::function::Function;
use php_codegen::parameter::Parameter;
use signal_hook::{
    consts::{SIGINT, SIGTERM},
    iterator::Signals,
};
use spinners::{Spinner, Spinners};
use std::{
    env,
    error::Error,
    fs,
    io::{self, BufRead},
    path::Path,
    process::Command,
    thread,
};
use subprocess::{Exec, Redirection};

mod generator;
mod observer;

pub fn make_action(kind: &String, name: &String) {
    println!("[elle] Generating {} action {}", kind, name);
    generator::make_action(kind, name);
}

pub fn make_migration_create(name: &String) {
    println!("[elle] Generating migration {}", name);
    generator::make_migration_create(name);
}

pub fn make_model(name: &String) {
    println!("[elle] Generating migration {}", name);
    generator::make_model(name);
}

pub fn new(name: &String) {
    let mut spinner = Spinner::new(Spinners::Aesthetic, "Crafting your new application".into());
    Command::new("cp")
        .arg("-r")
        .arg("./elle")
        .arg(&format!("./{}", name))
        .output()
        .expect("failed to copy elle/framework");
    Command::new("mkdir")
        .arg("-p")
        .arg(&format!("./{}/.elle/.laravel", name))
        .output()
        .expect("failed to create .elle directory");
    Command::new("cp")
        .arg("-r")
        .arg("./opinion/.")
        .arg(&format!("./{}/.elle/.laravel", name))
        .output()
        .expect("failed to copy elle/opinion");
    Command::new("touch")
        .arg(".gitignore")
        .output()
        .expect("failed to create .gitignore file");
    Command::new("cp")
        .arg("./cli/elle")
        .arg(&format!("./{}/", name))
        .output()
        .expect("failed to copy elle cli");
    Command::new("composer")
        .current_dir(&format!("./{}/", name))
        .arg("install")
        .output()
        .expect("unable to install composer dependencies in ellevel");
    Command::new("composer")
        .current_dir(&format!("./{}/.elle/.laravel", name))
        .arg("install")
        .output()
        .expect("unable to install composer dependencies in metavel");

    // Command::new("composer")
    //     .args(&[
    //         "create-project",
    //         "--prefer-dist",
    //         "elle/framework",
    //         &format!("./{}", name),
    //     ])
    //     .output()
    //     .expect("Failed to download elle/framework");
    // Command::new("composer")
    //     .args(&[
    //         "create-project",
    //         "--prefer-dist",
    //         "elle/opinion",
    //         &format!("./{}/.elle/.laravel", name),
    //     ])
    //     .output()
    //     .expect("Failed to download elle/opinion");
    spinner.stop_with_message("Application crafted!".into());
}

pub fn serve(host: String, port: u16) {
    if !fs::metadata(".elle").is_ok() || !fs::metadata(".elle/.laravel").is_ok() {
        println!("This is not an elle application. Did you mean to run [elle new]?");
        std::process::exit(1);
    }

    println!("Serving application @ http://{}:{}", host, port);

    start_server(host, port).expect("failed to start server");
}

fn start_server(host: String, port: u16) -> Result<(), Box<dyn Error>> {
    let mut signals = Signals::new(&[SIGINT, SIGTERM])?;

    let mut serve_process = Exec::cmd("php")
        .cwd(".elle/.laravel")
        .args(&[
            "artisan",
            "serve",
            "--host",
            &host.to_string(),
            "--port",
            &port.to_string(),
        ])
        .stdout(Redirection::Pipe)
        .popen()?;

    let stdout = serve_process.stdout.take().unwrap();

    thread::spawn(move || {
        let cwd = env::current_dir().expect("failed to get current directory");
        observer::start(&cwd.to_str().unwrap().to_string()).expect("failed to start observer");
    });

    thread::spawn(move || {
        let reader = io::BufReader::new(stdout);
        for _line in reader.lines() {
            // unwrap and parse the line for what we would like to display
            // println!(line);
        }
    });

    thread::spawn(move || {
        for _sig in signals.forever() {
            serve_process
                .terminate()
                .expect("Failed to terminate serve_process");
            std::process::exit(0);
        }
    });

    loop {}
}
