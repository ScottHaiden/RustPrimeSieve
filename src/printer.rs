use std::io::Read;
use std::io::BufReader;

fn main() {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin);

    loop {
        let mut block = [0u8; std::mem::size_of::<u32>()];
        if !reader.read_exact(&mut block).is_ok() { break; }
        let num = u32::from_be_bytes(block);
        println!("{:10} 0x{:08x}", num, num);
    }
}
