mod bloom;
mod hash_fns;
mod parse;
mod test;
fn main() {
    let hosts = parse::get_hosts();
    println!(">>> There are {} hosts", hosts.len());
    let mut bf = bloom::BloomFilter::new(2500000);

    for host in &hosts {
        bf.insert(&host);
    }

    for host in &hosts {
        let found = bf.contains(&host);
        if !found {
            println!("not found!");
        }
    }

    println!("Testing...");
    let rate = test::test(&bf, 10000);
    println!("False Positive Rate {rate}")
}
