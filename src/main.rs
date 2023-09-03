use std::io::Write;
use std::io::BufWriter;

fn is_prime(i: u32, primes: &Vec<u32>) -> bool {
    let limit = (i as f64).sqrt() as u32;
    for &prime in primes {
        if prime > limit { break; }
        if i % prime == 0 { return false; }
    }
    return true;
}

fn print_progress(i: u32, found: u32) {
    let mask = 0xffff;
    let masked = i & mask;
    if masked != mask { return; }
    let factor = 100f64 / (u32::MAX as f64);
    let percent = factor * i as f64;
    eprintln!("{:08x} ({:02.5}%) {:10}", i, percent, found);
}

fn main() {
    let stdout = std::io::stdout();
    let mut out = BufWriter::new(stdout);

    let max = u32::MAX;
    let max_track = (max as f64).sqrt() as u32;

    let mut found = 0u32;
    let mut primes = Vec::new();

    for i in (3..max).step_by(2) {
        print_progress(i, found);
        if !is_prime(i, &primes) { continue; }

        found += 1;
        let bytes = i.to_be_bytes();
        out.write_all(&bytes).expect("write failed");

        if i > max_track { continue; }
        primes.push(i);
    };
}
