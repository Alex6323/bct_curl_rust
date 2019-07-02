use curl_troika_benchmark::bct::multiplexer::*;
use curl_troika_benchmark::constants::*;
use curl_troika_benchmark::curl::bct_curl_128::*;
use curl_troika_benchmark::curl::bct_curl_64::*;
use curl_troika_benchmark::curl::bct_fcurl_128::*;
use curl_troika_benchmark::curl::bct_fcurl_64::*;
use curl_troika_benchmark::curl::curl::Curl;
use curl_troika_benchmark::curl::fcurl::fcurl;
use curl_troika_benchmark::troika::*;
use curl_troika_benchmark::xxhash::*;

use std::sync::atomic::{
    AtomicUsize,
    Ordering,
};
use std::time::{
    Duration,
    Instant,
};

use num_cpus;
use rand::distributions::Uniform;
use rand::Rng;
use scoped_pool::Pool;

// TODO: Determine processor

fn main() {
    println!(
        "+-------------------------------------------------------------------------+"
    );
    println!(
        "| This tool compares several different IOTA hashing functions, namely     |"
    );
    println!(
        "| various Curl and Troika implementations.                                |"
    );
    println!(
        "| As a baseline serves xxHash, a very fast commonly used hashing function.|"
    );
    println!(
        "|                                                                         |"
    );
    println!(
        "| Disclaimer: These benchmarks do not satisfy scientific standards and    |"
    );
    println!(
        "| are just intended to get a rough idea about the performance of each     |"
    );
    println!(
        "| implementation.                                                         |"
    );
    println!(
        "|                                                                         |"
    );
    println!("|{:^73}|", format!("Curl-P-{}", NUM_CURL_ROUNDS));
    println!("|{:^73}|", format!("Troika-{}", NUM_TROIKA_ROUNDS));
    println!("|{:^73}|", format!("Workload: {} transactions", NUM_HASHES));
    println!(
        "+-------------------------------------------------------------------------+"
    );
    println!();

    bench_curl(NUM_HASHES);
    bench_curl_par(NUM_HASHES);
    bench_functional_curl(NUM_HASHES);
    bench_functional_curl_par(NUM_HASHES);
    bench_bct_curl_64(NUM_HASHES);
    bench_bct_curl_128(NUM_HASHES);
    bench_functional_bct_curl_64(NUM_HASHES);
    bench_functional_bct_curl_128(NUM_HASHES);
    bench_functional_bct_curl_64_par(NUM_HASHES);
    //bench_simd_functional_bct_curl_64(NUM_HASHES);
    //bench_simd_functional_bct_curl_64_par(NUM_HASHES);
    bench_xxhash(NUM_HASHES);
    bench_xxhash_par(NUM_HASHES);
    bench_troika(NUM_HASHES);
}

fn bench_curl(num_hashes: usize) {
    print_title("Curl (1 thread)");

    let transactions_as_trits = get_random_tx_balanced_trits(num_hashes);
    let mut curl = Curl::default();
    let mut hash_trits = [0i8; HASH_LENGTH];

    let start = Instant::now();
    for i in 0..transactions_as_trits.len() {
        curl.absorb(&transactions_as_trits[i], 0, TRANSACTION_TRIT_LENGTH);
        curl.squeeze(&mut hash_trits, 0, HASH_LENGTH);
        curl.reset();
    }
    print_timing(start.elapsed(), num_hashes);
}

fn bench_curl_par(num_hashes: usize) {
    print_title(&format!("Curl ({} threads)", num_cpus::get()));

    let transactions_as_trits = get_random_tx_balanced_trits(num_hashes);
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

    print_timing(start.elapsed(), num_hashes);
}

fn bench_functional_curl(num_hashes: usize) {
    print_title("Functional Curl (1 thread)");

    let transactions_as_trits = get_random_tx_balanced_trits(num_hashes);

    let start = Instant::now();
    for i in 0..transactions_as_trits.len() {
        let _hash_trits = fcurl(&transactions_as_trits[i], NUM_CURL_ROUNDS);
    }

    print_timing(start.elapsed(), num_hashes);
}
fn bench_functional_curl_par(num_hashes: usize) {
    print_title(&format!("Functional Curl ({} threads)", num_cpus::get()));

    let transactions_as_trits = get_random_tx_balanced_trits(num_hashes);
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
                    let _hash_trits = fcurl(&transactions_as_trits[j], NUM_CURL_ROUNDS);
                }
            })
        }
    });

    print_timing(start.elapsed(), num_hashes);
}

fn bench_bct_curl_64(num_hashes: usize) {
    print_title("BCT-Curl-64 (1 thread)");

    let transactions_as_trits = get_random_tx_balanced_trits(num_hashes);

    let mut bct_curl = BCTCurl64::new(HASH_LENGTH, NUM_CURL_ROUNDS);
    let mut bct_mux = BCTMultiplexer::default();

    let start = Instant::now();
    for i in 0..transactions_as_trits.len() {
        bct_mux.add(&transactions_as_trits[i]);

        if bct_mux.len() == 64 {
            bct_curl.absorb(bct_mux.extract64());
            let demux = BCTDemultiplexer64::from(bct_curl.squeeze(HASH_LENGTH));
            for index in 0..64 {
                let _hash_trits = demux.get(index);
            }

            bct_curl.reset();
            bct_mux.reset();
            //println!("{}", i);
            //assert_eq!(0, bct_mux.len());
        }
    }

    print_timing(start.elapsed(), num_hashes);
}

fn bench_bct_curl_128(num_hashes: usize) {
    print_title("BCT-Curl-128 (1 thread)");

    let transactions_as_trits = get_random_tx_balanced_trits(num_hashes);

    let mut bct_curl = BCTCurl128::new(HASH_LENGTH, NUM_CURL_ROUNDS);
    let mut bct_mux = BCTMultiplexer::default();

    let start = Instant::now();
    for i in 0..transactions_as_trits.len() {
        bct_mux.add(&transactions_as_trits[i]);

        if bct_mux.len() == 128 {
            bct_curl.absorb(bct_mux.extract128());
            let demux = BCTDemultiplexer128::from(bct_curl.squeeze(HASH_LENGTH));
            for index in 0..128 {
                let _hash_trits = demux.get(index);
            }
            bct_curl.reset();
            bct_mux.reset();
        }
    }
    print_timing(start.elapsed(), num_hashes);
}

fn bench_functional_bct_curl_64(num_hashes: usize) {
    print_title("Functional BCT-Curl/64 (1 thread)");

    let transactions_as_trits = get_random_tx_balanced_trits(num_hashes);

    let start = Instant::now();
    let _hash_trits = bct_fcurl_64(&transactions_as_trits, NUM_CURL_ROUNDS);

    print_timing(start.elapsed(), num_hashes);
}

fn bench_functional_bct_curl_128(num_hashes: usize) {
    print_title("Functional BCT-Curl-128 (1 thread)");

    let transactions_as_trits = get_random_tx_balanced_trits(num_hashes);

    let start = Instant::now();
    let _hash_trits = bct_fcurl_128(&transactions_as_trits, NUM_CURL_ROUNDS);

    print_timing(start.elapsed(), num_hashes);
}

fn bench_functional_bct_curl_64_par(num_hashes: usize) {
    print_title(&format!("Functional BCT-Curl-64 ({} threads)", num_cpus::get()));

    let transactions_as_trits = get_random_tx_balanced_trits(num_hashes);

    let start = Instant::now();
    let _hash_trits = bct_fcurl_64_par(&transactions_as_trits, NUM_CURL_ROUNDS);

    print_timing(start.elapsed(), num_hashes);
}

fn bench_simd_functional_bct_curl_64(num_hashes: usize) {
    print_title("SIMD Functional BCT-Curl-64 (1 thread)");

    let transactions_as_trits = get_random_tx_balanced_trits(num_hashes);

    let start = Instant::now();
    //let _hash_trits = simd_bct_fcurl_64(&transactions_as_trits, NUM_CURL_ROUNDS);

    print_timing(start.elapsed(), num_hashes);
}

fn bench_simd_functional_bct_curl_64_par(num_hashes: usize) {
    print_title(&format!("SIMD Functional BCT-Curl-64 ({} threads)", num_cpus::get()));

    let transactions_as_trits = get_random_tx_balanced_trits(num_hashes);

    let start = Instant::now();
    //let _hash_trits = simd_bct_fcurl_64_par(&transactions_as_trits, NUM_CURL_ROUNDS);

    print_timing(start.elapsed(), num_hashes);
}

fn bench_xxhash(num_hashes: usize) {
    print_title("xxHash (1 thread)");

    let transactions_as_trits = get_random_tx_balanced_trits(num_hashes);

    let start = Instant::now();
    for i in 0..transactions_as_trits.len() {
        let _hash = xxhash(&transactions_as_trits[i]);
    }
    print_timing(start.elapsed(), num_hashes);
}

fn bench_xxhash_par(num_hashes: usize) {
    print_title(&format!("xxHash ({} threads)", num_cpus::get()));

    let transactions_as_trits = get_random_tx_balanced_trits(num_hashes);
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
                    let _hash = xxhash(&transactions_as_trits[j]);
                }
            })
        }
    });

    print_timing(start.elapsed(), num_hashes);
}

fn bench_troika(num_hashes: usize) {
    print_title("Troika (1 thread)");

    let transactions_as_trits = get_random_tx_unbalanced_trits(num_hashes);
    let mut hash_trits = [0; 243];

    let start = Instant::now();
    for i in 0..transactions_as_trits.len() {
        troika::troika_var_rounds(
            &mut hash_trits,
            &transactions_as_trits[i],
            NUM_TROIKA_ROUNDS,
        );
    }

    print_timing(start.elapsed(), num_hashes);
}

// Helper function to generate random transaction trits
macro_rules! make_random_trits_fn {
    ($FUNC: ident, $TYPE: ty, $RANGE: expr) => {
        fn $FUNC(count: usize) -> Vec<Vec<$TYPE>> {
            let mut tx_trits = vec![];
            let trit = Uniform::from($RANGE);
            let rng = rand::thread_rng();

            for _ in 0..count {
                let trits = rng
                    .sample_iter(&trit)
                    .take(TRANSACTION_TRIT_LENGTH)
                    .collect::<Vec<$TYPE>>();
                tx_trits.push(trits);
            }

            tx_trits
        }
    };
}

make_random_trits_fn!(get_random_tx_unbalanced_trits, u8, 0..3);
make_random_trits_fn!(get_random_tx_balanced_trits, i8, -1..2);

fn print_title(title: &str) {
    use std::io::Write;
    print!("{:<40} ", title);
    std::io::stdout().flush().unwrap();
}

// Helper function to print results
fn print_timing(stop: Duration, num_hashes: usize) {
    let ms = stop.as_secs() * 1000 + u64::from(stop.subsec_millis());
    let hps = num_hashes as f64 / ms as f64 * 1000.0;
    println!("{:>10} ms {:>10.0} hashes/s", ms, hps);
}
