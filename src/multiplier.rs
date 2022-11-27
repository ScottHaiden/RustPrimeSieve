use gmp::mpz::Mpz;
use std::io::{Read, Write};

type Block = [u8; 4096];

fn multiply_all(block: &Block) -> Mpz {
    let size = std::mem::size_of::<u32>();
    let max = std::mem::size_of::<Block>() / size;

    let mut ret = Mpz::one();
    for i in 0..max {
        let offset = i * size;
        let buf = &block[offset..offset + size];
        let cur = u32::from_be_bytes(buf.try_into().unwrap());
        if cur == 0 { break; }
        ret *= cur as i64;
    }

    return ret;
}

fn read_block() -> Option<Block> {
    let mut stdin = std::io::stdin();

    let bytes = std::mem::size_of::<Block>();
    let size = std::mem::size_of::<u32>();
    let ints = bytes / size;

    let mut ret: Block = [0u8; std::mem::size_of::<Block>()];

    let mut read_any: bool = false;
    for i in 0..ints {
        let mut block = [0u8; std::mem::size_of::<u32>()];
        if !stdin.read_exact(&mut block).is_ok() { break; }
        let offset = size * i;
        let buf = &mut ret[offset..offset + size];
        buf.copy_from_slice(&block);
        read_any = true;
    }

    if read_any {
        return Some(ret);
    } else {
        return None;
    }
}

fn main() {
    let nmults = std::thread::available_parallelism().unwrap().get();

    let (read, mut write) = socketpair::socketpair_seqpacket().unwrap();

    let mut handles = Vec::new();
    for _ in 0..nmults {
        let mut local_read = read.try_clone().unwrap();
        handles.push(std::thread::spawn(move || -> Mpz {
            let mut product = Mpz::one();
            loop {
                let mut block = [0u8; std::mem::size_of::<Block>()];
                let amount = match local_read.read(&mut block) {
                    Ok(n) => n,
                    Err(e) => panic!("Failed to read: {}", e),
                };
                if amount % 4 != 0 {
                    panic!("Did not read a multiple of 4!");
                }
                if amount == 0 { break; }
                product *= multiply_all(&block);
            }
            return product;
        }));
    }

    loop {
        let block = match read_block() {
            Some(block) => block,
            None => break,
        };
        let written = match write.write(&block) {
            Ok(n) => n,
            Err(e) => panic!("Write failed! {}", e),
        };
        if written != std::mem::size_of::<Block>() {
            panic!("wrote {} instead of {}", written, std::mem::size_of::<Block>());
        }
    }

    drop(write);

    let mut num = Mpz::one();
    let mut n = 0;
    for handle in handles {
        let cur = match handle.join() {
            Ok(n) => n,
            Err(_) => panic!("Join failed"),
        };
        eprintln!("read handle {} / {}...", n + 1, nmults);
        num *= cur;
        n += 1;
    }

    eprintln!("Stringifying the primorial...");

    println!("{}", num);
}
