use std::{
    thread::{self, sleep},
    time::Duration,
};

use rand::{RngCore, SeedableRng};

fn main() {
    let size = std::env::args()
        .nth(1)
        .unwrap_or("1G".to_string())
        .trim()
        .to_string();
    let unit = size.chars().last().unwrap();
    let size = size[..size.len() - 1].parse::<u64>().unwrap();
    let size = match unit {
        'G' => size * 1024 * 1024 * 1024,
        'M' => size * 1024 * 1024,
        'K' => size * 1024,
        _ => size,
    };

    let threads = std::env::args()
        .nth(2)
        .unwrap_or("8".to_string())
        .trim()
        .to_string();

    // Spawn 8 threads to fill the memory
    for i in 0..8 {
        thread::spawn(move || {
            println!("{i} Allocating {} bytes", size);
            let mut memory = vec![0u8; size as usize];

            println!("{i} Filling memory with stable random data");

            let seed = [0xffu8; 32];

            let mut rng = rand::rngs::StdRng::from_seed(seed);

            for i in (0..memory.len()).step_by(8) {
                let current = rng.next_u64();
                memory[i + 0] = (current >> 0) as u8;
                memory[i + 1] = (current >> 8) as u8;
                memory[i + 2] = (current >> 16) as u8;
                memory[i + 3] = (current >> 24) as u8;
                memory[i + 4] = (current >> 32) as u8;
                memory[i + 5] = (current >> 40) as u8;
                memory[i + 6] = (current >> 48) as u8;
                memory[i + 7] = (current >> 56) as u8;
            }

            // Basic checksum
            let checksum = memory
                .iter()
                .fold(0u64, |acc, &x| acc.wrapping_add(x as u64));
            println!("{i} Checksum: {:#X}", checksum);
            loop {
                sleep(Duration::from_millis(100));
            }
        });
    }

    loop {
        sleep(Duration::from_millis(100));
    }
}
