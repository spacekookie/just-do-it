extern crate clap;
extern crate colored;
extern crate num_cpus;
extern crate question;

#[macro_use]
extern crate human_panic;

use clap::{App, AppSettings, Arg, SubCommand};
use question::{Answer, Question};
use std::process::{self, Command};
use colored::*;

const DELETE_QUESTION: &'static str =
    "Are you sure you want to destroy 🔥 (and murder 🔪) this container and all of it's happy friends? ";

fn main() {
    setup_panic!();

    let m = App::new("doit")
        .setting(AppSettings::SubcommandRequired)
        .about("It works for me, okay?")
        .subcommand(
            SubCommand::with_name("new")
                .display_order(1)
                .about("Create new containers and install basic dependencies")
                .arg(Arg::with_name("name").takes_value(true).required(true)),
        )
        .subcommand(
            SubCommand::with_name("work")
                .display_order(2)
                .about("Start and attach a container to work with it")
                .arg(Arg::with_name("name").takes_value(true).required(true))
                .arg(
                    Arg::with_name("shutdown")
                        .default_value("no")
                        .possible_values(&["yes", "no", "hell no"])
                        .help("Shutdown this container after working on it"),
                ),
        )
        .subcommand(
            SubCommand::with_name("list")
                .display_order(3)
                .about("Show existing containers and stuff"),
        )
        .subcommand(
            SubCommand::with_name("destroy")
                .display_order(4)
                .about("Murder existing containers & their families in cold bits...")
                .arg(Arg::with_name("name").takes_value(true).required(true)),
        );

    match m.get_matches().subcommand() {
        ("new", Some(m)) => {
            let name = m.value_of("name").unwrap();
            println!(
                "Creating a new container from scratch, with my bare {} cores!",
                num_cpus::get()
            );
            docker_create(name);
            docker_start(name);
            docker_base_install(name);
            println!("✨✨✨ Container created and ready: {} ✨✨✨", name);
        }
        ("work", Some(m)) => {
            let name = m.value_of("name").unwrap();
            println!(
                "Look at you busy bee, let's get you ready working on ✨ {} ✨",
                name
            );
            docker_start(name);
            docker_attach(name);
        }
        ("list", _) => docker_list_all(),

        ("destroy", Some(m)) => {
            let name = m.value_of("name").unwrap();
            let answer = Question::new(DELETE_QUESTION)
                .default(Answer::NO)
                .show_defaults()
                .confirm();

            if answer == Answer::NO {
                process::exit(2);
            }

            docker_stop(name);
            docker_delete(name);

            println!("Deleted container {} 🔥🔥🔥", name);
        }
        _ => unreachable!(),
    }
}

fn docker_list_all() {
    let args = ["container", "list", "--all"];
    let mut cmd = Command::new("docker")
        .args(&args)
        .spawn()
        .expect(&format!("docker list failed for {:?}", &args));

    if !cmd.wait().unwrap().success() {
        eprintln!("{}", "Failed to run list command!".red());
        process::exit(2);
    }
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
        eprintln!("{}", "Failed to run create command!".red());
        process::exit(2);
    }
}

/// Only starts IF ABSOLUTELY NECESSARY
///
/// Okay I'm being more dramatic than neccessary.
///
/// O R    A M     I
fn docker_start(name: &str) {
    let args = ["start", name];
    let mut cmd = Command::new("docker")
        .args(&args)
        .spawn()
        .expect(&format!("docker start failed for {:?}", &args));

    if !cmd.wait().unwrap().success() {
        eprintln!("{}", "Failed to run start command!".red());
        process::exit(2);
    }
}

fn docker_stop(name: &str) {
    let args = ["stop", name];
    let mut cmd = Command::new("docker")
        .args(&args)
        .spawn()
        .expect(&format!("docker stop failed for {:?}", &args));

    if !cmd.wait().unwrap().success() {
        eprintln!("{}", "Failed to run stop command!".red());
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
        eprintln!("{}", "Failed to run delete command!".red());
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
        eprintln!("{}", "Failed to run install command!".red());
        process::exit(2);
    }
}

/// This doesn't return
fn docker_attach(name: &str) {
    let args = ["attach", name];
    let mut cmd = Command::new("docker")
        .args(&args)
        .spawn()
        .expect(&format!("docker start failed for {:?}", &args));

    if !cmd.wait().unwrap().success() {
        eprintln!("{}", "Failed to run start command!".red());
        process::exit(2);
    }
}
