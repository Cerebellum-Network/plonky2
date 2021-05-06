use std::thread;
use std::time::Instant;

use plonky2::field::crandall_field::CrandallField;
use plonky2::field::field::Field;
use plonky2::gmimc::gmimc_permute_array;
use plonky2::hash::{GMIMC_CONSTANTS, GMIMC_ROUNDS};

type F = CrandallField;

// 113 wire polys, 3 Z polys, 4 parts of quotient poly.
const PROVER_POLYS: usize = 113 + 3 + 4;

fn main() {
    const THREADS: usize = 12;
    const LDE_BITS: i32 = 3;
    const W: usize = 12;
    const HASHES_PER_POLY: usize = 1 << (13 + LDE_BITS) / 6;

    let threads = (0..THREADS)
        .map(|_i| {
            thread::spawn(move || {
                let mut x = [F::ZERO; W];
                for i in 0..W {
                    x[i] = F::from_canonical_u64((i as u64) * 123456 + 789);
                }

                let hashes_per_thread = HASHES_PER_POLY * PROVER_POLYS / THREADS;
                let start = Instant::now();
                for _ in 0..hashes_per_thread {
                    x = gmimc_permute_array::<_, W, GMIMC_ROUNDS>(x, GMIMC_CONSTANTS);
                }
                let duration = start.elapsed();
                println!("took {:?}", duration);
                println!(
                    "avg {:?}us",
                    duration.as_secs_f64() * 1e6 / (hashes_per_thread as f64)
                );
                println!("result {:?}", x);
            })
        })
        .collect::<Vec<_>>();

    for t in threads {
        t.join().expect("oops");
    }
}