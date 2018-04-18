extern crate clap;
extern crate question;
#[macro_use]
extern crate human_panic;
use clap::{App, Arg, SubCommand};
use question::{Answer, Question};
use std::process::{self, Command};

const DELETE_QUESTION: &'static str =
    "Are you sure you want to destroy 🔥 (and murder 🔪) this container and all of it's happy friends? ";

fn main() {
    setup_panic!();

    let m = App::new("doit")
        .about("It works for me, okay?")
        .subcommand(
            SubCommand::with_name("new")
                .about("Create new containers and install basic dependencies")
                .arg(Arg::with_name("name").takes_value(true).required(true)),
        )
        .subcommand(
            SubCommand::with_name("destroy")
                .about("Murder existing containers & their families in cold bits...")
                .arg(Arg::with_name("name").takes_value(true).required(true)),
        );

    match m.get_matches().subcommand() {
        ("new", Some(m)) => {
            let name = m.value_of("name").unwrap();
            docker_create(name);
            docker_base_install(name);
            println!("✨✨✨ Container created and ready: {} ✨✨✨", name);
        }
        ("destroy", Some(m)) => {
            let name = m.value_of("name").unwrap();
            let answer = Question::new(DELETE_QUESTION)
                .default(Answer::NO)
                .show_defaults()
                .confirm();

            if answer == Answer::NO {
                process::exit(2);
            }

            docker_delete(name);

            println!("Deleted container {} 🔥🔥🔥", name);
        }
        _ => unreachable!(),
    }

    println!("Hello, world!");
}

fn docker_create(name: &str) {
    let args = [
        "create",
        "--name",
        name,
        "-t",
        "-i",
        "fedora:latest",
        "bash",
    ];
    let mut cmd = Command::new("docker")
        .args(&args)
        .spawn()
        .expect(&format!("docker create failed for {:?}", &args));

    if !cmd.wait().unwrap().success() {
        eprintln!("Failed to run create command!");
        process::exit(2);
    }
}

fn docker_delete(name: &str) {
    let args = ["rm", name];
    let mut cmd = Command::new("docker")
        .args(&args)
        .spawn()
        .expect(&format!("docker delete failed for {:?}", &args));

    if !cmd.wait().unwrap().success() {
        eprintln!("Failed to run delete command!");
        process::exit(2);
    }
}

/// Run a certain amount of base install stuff on a container
///
/// See the code for what tools are included
fn docker_base_install(name: &str) {
    let args = [
        "exec",
        name,
        "dnf",
        "install",
        "-y",
        "@development-tools",
        "gpg",
        "which",
        "curl",
        "wget",
        "vim",
        "fish",
        "openssh",
        "sshfs",
    ];

    let mut cmd = Command::new("docker")
        .args(&args)
        .spawn()
        .expect(&format!("docker base install failed for {:?}", args));

    if !cmd.wait().unwrap().success() {
        eprintln!("Failed to run install command!");
        process::exit(2);
    }
}
