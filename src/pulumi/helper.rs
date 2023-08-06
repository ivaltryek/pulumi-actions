use std::{
    env,
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};

pub fn set_env_var(key: &String, value: &String) {
    env::set_var(key, value)
}

pub fn run_cmd_print_output(arg_str: &String) {
    let cmd = Command::new("/bin/bash")
        .args(["-c", &arg_str])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap()
        .stdout
        .ok_or_else(|| "Could not parse the stdout")
        .unwrap();

    let reader = BufReader::new(cmd);
    reader
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| println!("{}", line));
}
