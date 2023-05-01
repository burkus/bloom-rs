use std::fs;

fn read_hosts() -> String {
    let content = fs::read_to_string("hosts.txt").expect("Hosts file to exist");
    content
}

pub fn get_hosts() -> Vec<String> {
    read_hosts()
        .lines()
        .map(|line| line.to_string())
        .filter(|line| line.starts_with("0.0.0.0"))
        .collect::<Vec<String>>()
}
