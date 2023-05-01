use crate::bloom::BloomFilter;
use rand::Rng;

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

fn random_string(length: u8) -> String {
    let mut rng = rand::thread_rng();
    let mut s = String::from("");
    for _ in 0..length {
        let i: usize = rng.gen::<u8>() as usize % 26;
        s.push(ALPHABET.chars().nth(i).unwrap());
    }
    s
}

pub fn random_host(domains: u8, length: u8) -> String {
    let mut s = String::from("");
    for _ in 0..domains {
        let domain = random_string(length);
        s = format!("{}.{}", s, domain);
    }
    s
}

pub fn random_hosts(num: u32, length: u8) -> Vec<String> {
    let mut vec = Vec::new();
    for _ in 0..num {
        vec.push(random_host(3, length));
    }
    vec
}

pub fn test(filter: &BloomFilter, num: u32) -> u32 {
    let hosts = random_hosts(num, 10);
    let mut rate = 0;
    for host in hosts {
        let found = filter.contains(&host);
        if found {
            rate += 1
        }
    }

    rate / num
}
