use std::process::{Command,exit};

use names::Generator;




fn update_commit_push(){
    let add_command=Command::new("git")
        .arg("add")
        .arg(".")
        .output()
        .expect("Git add Command Failed");

    if !add_command.status.success(){
        eprintln!("Error failed to add files to git");
        exit(1);
    }


    let commit_command=Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(name_generator())
        .output()
        .expect("Commit Failed");
    if !commit_command.status.success(){
        eprintln!("Failed to commit files to git");
        exit(1);
    }

    let push_command=Command::new("git")
        .arg("push")
        .arg("origin")
        .arg("-u")
        .arg("main")
        .output()
        .expect("Failed to run push_command");

    if !push_command.status.success(){
        eprintln!("Failed to push files to github");
        exit(1);
    }
        

}

fn name_generator()->String{
    let mut message=Generator::default();
    message.next().unwrap()
}

fn main() {
    update_commit_push();
}
