use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

use num_cpus;
use rand::distributions::Uniform;
use rand::Rng;
use scoped_pool::Pool;

use bct_curl_rust::{
    bct_curl_128::*, bct_curl_64::*, bct_fcurl::*, bct_fcurl128::*, bct_mux::*, constants::*,
    curl::*, fcurl::*, simd_bct_fcurl::*, xxhash::*,
};

const NUM_HASHES: usize = 5000;

fn main() {
    let r = NUM_ROUNDS;
    let h = NUM_HASHES;

    println!("+-------------------------------------------------------------------------+");
    println!("| This tool compares several Curl implementations in Rust. For reference, |");
    println!("| I also included the xxHash hashing algorithm.                           |");
    println!("|                                                                         |");
    println!("| Disclaimer: What follows are some completely unscientific benchmarks,   |");
    println!("| and is just intended to give you a rough idea about how the various     |");
    println!("| Curl implementations compare to eachother.                              |");
    println!("|                                                                         |");
    println!("| You might want to run this tool several times as the results vary a lot |");
    println!("| depending on how busy your machine is at the moment.                    |");
    println!("|                                                                         |");
    println!("|{:^73}|", format!("Algorithm: Curl-P-{}", NUM_ROUNDS));
    println!("|{:^73}|", format!("Workload: {} hashes", NUM_HASHES));
    println!("+-------------------------------------------------------------------------+");
    println!();
    println!("Fighting the cold...");
    println!();

    bench_curl(NUM_HASHES);
    bench_curl_par(NUM_HASHES);
    bench_fcurl(NUM_HASHES);
    bench_fcurl_par(NUM_HASHES);
    bench_bct_curl_64(NUM_HASHES);
    bench_bct_curl_128(NUM_HASHES);
    bench_bct_fcurl_64(NUM_HASHES);
    bench_bct_fcurl_128(NUM_HASHES);
    bench_bct_fcurl_64_par(NUM_HASHES);
    bench_simd_bct_fcurl_64(NUM_HASHES);
    bench_simd_bct_fcurl_64_par(NUM_HASHES);
    bench_xxhash(NUM_HASHES);
    bench_xxhash_par(NUM_HASHES);
}

fn bench_xxhash(num_hashes: usize) {
    let transactions_as_trits = get_random_tx_trits(num_hashes);

    let start = Instant::now();
    for i in 0..transactions_as_trits.len() {
        let hash = xxhash(&transactions_as_trits[i]);
    }
    print_timing("xxHash (1 thread)", start.elapsed(), num_hashes);
}

fn bench_xxhash_par(num_hashes: usize) {
    let transactions_as_trits = get_random_tx_trits(num_hashes);
    let num_threads = num_cpus::get();
    let pool = Pool::new(num_threads);
    let chunk_length = transactions_as_trits.len() / num_threads;
    let index = AtomicUsize::new(0);

    let start = Instant::now();
    pool.scoped(|scope| {
        for _ in 0..num_threads {
            scope.execute(|| {
                let i = index.fetch_add(1, Ordering::SeqCst);
                let offset = i * chunk_length;
                for j in offset..offset + chunk_length {
                    let hash = xxhash(&transactions_as_trits[j]);
                }
            })
        }
    });
    print_timing(
        &format!("xxHash ({} threads)", num_cpus::get()),
        start.elapsed(),
        num_hashes,
    );
}

fn bench_curl(num_hashes: usize) {
    let transactions_as_trits = get_random_tx_trits(num_hashes);
    let mut curl = Curl::default();
    let mut hash_trits = [0i8; HASH_LENGTH];

    let start = Instant::now();
    for i in 0..transactions_as_trits.len() {
        curl.absorb(&transactions_as_trits[i], 0, TRANSACTION_TRIT_LENGTH);
        curl.squeeze(&mut hash_trits, 0, HASH_LENGTH);
        curl.reset();
    }
    print_timing("Curl (1 thread)", start.elapsed(), num_hashes);
}

fn bench_fcurl(num_hashes: usize) {
    let transactions_as_trits = get_random_tx_trits(num_hashes);

    let start = Instant::now();
    for i in 0..transactions_as_trits.len() {
        let hash_trits = fcurl(&transactions_as_trits[i], NUM_ROUNDS);
    }

    print_timing("Stateless Curl (1 thread)", start.elapsed(), num_hashes);
}

fn bench_curl_par(num_hashes: usize) {
    let transactions_as_trits = get_random_tx_trits(num_hashes);
    let num_threads = num_cpus::get();
    let pool = Pool::new(num_threads);
    let chunk_length = transactions_as_trits.len() / num_threads;
    let index = AtomicUsize::new(0);

    let start = Instant::now();
    pool.scoped(|scope| {
        for _ in 0..num_threads {
            scope.execute(|| {
                let i = index.fetch_add(1, Ordering::SeqCst);
                //
                let mut curl = Curl::default();
                let mut hash_trits = [0i8; HASH_LENGTH];
                let offset = i * chunk_length;
                //println!("offset {} = {}", index, offset);
                for j in offset..offset + chunk_length {
                    curl.absorb(&transactions_as_trits[j], 0, TRANSACTION_TRIT_LENGTH);
                    curl.squeeze(&mut hash_trits, 0, HASH_LENGTH);
                    curl.reset();
                }
            })
        }
    });
    print_timing(
        &format!("Curl ({} threads)", num_cpus::get()),
        start.elapsed(),
        num_hashes,
    );
}

fn bench_fcurl_par(num_hashes: usize) {
    let transactions_as_trits = get_random_tx_trits(num_hashes);
    let num_threads = num_cpus::get();
    let pool = Pool::new(num_threads);
    let chunk_length = transactions_as_trits.len() / num_threads;
    let index = AtomicUsize::new(0);

    let start = Instant::now();
    pool.scoped(|scope| {
        for _ in 0..num_threads {
            scope.execute(|| {
                let i = index.fetch_add(1, Ordering::SeqCst);
                //
                let offset = i * chunk_length;
                //println!("offset {} = {}", i, offset);
                for j in offset..offset + chunk_length {
                    let hash_trits = fcurl(&transactions_as_trits[j], NUM_ROUNDS);
                }
            })
        }
    });
    print_timing(
        &format!("Stateless Curl ({} threads)", num_cpus::get()),
        start.elapsed(),
        num_hashes,
    );
}

fn bench_bct_curl_64(num_hashes: usize) {
    let transactions_as_trits = get_random_tx_trits(num_hashes);

    let mut bct_curl = BCTCurl64::new(HASH_LENGTH, NUM_ROUNDS);
    let mut bct_mux = BCTMultiplexer::default();

    let start = Instant::now();
    for i in 0..transactions_as_trits.len() {
        bct_mux.add(&transactions_as_trits[i]);

        if bct_mux.len() == 64 {
            bct_curl.absorb(bct_mux.extract64());
            let demux = BCTDemultiplexer64::from(bct_curl.squeeze(HASH_LENGTH));
            for index in 0..64 {
                let hash_trits = demux.get(index);
            }

            bct_curl.reset();
            bct_mux.reset();
            //println!("{}", i);
            //assert_eq!(0, bct_mux.len());
        }
    }
    print_timing("BCT-Curl-64 (1 thread)", start.elapsed(), num_hashes);
}

fn bench_bct_curl_128(num_hashes: usize) {
    let transactions_as_trits = get_random_tx_trits(num_hashes);

    let mut bct_curl = BCTCurl128::new(HASH_LENGTH, NUM_ROUNDS);
    let mut bct_mux = BCTMultiplexer::default();

    let start = Instant::now();
    for i in 0..transactions_as_trits.len() {
        bct_mux.add(&transactions_as_trits[i]);

        if bct_mux.len() == 128 {
            bct_curl.absorb(bct_mux.extract128());
            let demux = BCTDemultiplexer128::from(bct_curl.squeeze(HASH_LENGTH));
            for index in 0..128 {
                let hash_trits = demux.get(index);
            }
            bct_curl.reset();
            bct_mux.reset();
        }
    }
    print_timing("BCT-128-Curl (1 thread)", start.elapsed(), num_hashes);
}

fn bench_bct_fcurl_64(num_hashes: usize) {
    let transactions_as_trits = get_random_tx_trits(num_hashes);

    let start = Instant::now();
    let hash_trits = bct_fcurl_64(&transactions_as_trits, NUM_ROUNDS);
    print_timing(
        "Stateless BCT-64-Curl (1 thread)",
        start.elapsed(),
        num_hashes,
    );
}

fn bench_bct_fcurl_128(num_hashes: usize) {
    let transactions_as_trits = get_random_tx_trits(num_hashes);

    let start = Instant::now();
    let hash_trits = bct_fcurl_128(&transactions_as_trits, NUM_ROUNDS);
    print_timing(
        "Stateless BCT-128-Curl (1 thread)",
        start.elapsed(),
        num_hashes,
    );
}

fn bench_bct_fcurl_64_par(num_hashes: usize) {
    let transactions_as_trits = get_random_tx_trits(num_hashes);

    let start = Instant::now();
    let hash_trits = bct_fcurl_64_par(&transactions_as_trits, NUM_ROUNDS);
    print_timing(
        &format!("Stateless BCT-64-Curl ({} threads)", num_cpus::get()),
        start.elapsed(),
        num_hashes,
    );
}

fn bench_simd_bct_fcurl_64(num_hashes: usize) {
    let transactions_as_trits = get_random_tx_trits(num_hashes);

    let start = Instant::now();
    let hash_trits = simd_bct_fcurl_64(&transactions_as_trits, NUM_ROUNDS);
    print_timing(
        "Stateless SIMD-BCT-64-Curl (1 thread)",
        start.elapsed(),
        num_hashes,
    );
}

fn bench_simd_bct_fcurl_64_par(num_hashes: usize) {
    let transactions_as_trits = get_random_tx_trits(num_hashes);

    let start = Instant::now();
    let hash_trits = simd_bct_fcurl_64_par(&transactions_as_trits, NUM_ROUNDS);

    print_timing(
        &format!("Stateless SIMD-BCT-64-Curl ({} threads)", num_cpus::get()),
        start.elapsed(),
        num_hashes,
    );
}

// Helper function to generate random transaction trits
fn get_random_tx_trits(count: usize) -> Vec<Vec<i8>> {
    let mut tx_trits = vec![];
    let trit = Uniform::from(-1..2);
    let mut rng = rand::thread_rng();

    for _ in 0..count {
        let trits = rng
            .sample_iter(&trit)
            .take(TRANSACTION_TRIT_LENGTH)
            .collect::<Vec<i8>>();
        tx_trits.push(trits);
    }

    tx_trits
}

// Helper function to print results
fn print_timing(title: &str, stop: Duration, num_hashes: usize) {
    let ms = stop.as_secs() * 1000 + u64::from(stop.subsec_millis());
    let hps = num_hashes as f64 / ms as f64 * 1000.0;
    println!("{:<40} {:>10} ms {:>10.0} hashes/s", title, ms, hps);
}
