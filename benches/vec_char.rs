// benches/my_benchmark.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::{rngs::ThreadRng, Rng};

// Perform multiple deletions on a Vec<char>
fn remove_slices_vec(mut s: Vec<char>, operations: &[(usize, usize)]) -> Vec<char> {
    for &(start, end) in operations.iter().rev() {
        if start < end && end <= s.len() {
            s.drain(start..end);
        }
    }
    s
}

// Perform multiple deletions on a Vec<char>
fn remove_slices_vec_ref(s: &mut Vec<char>, operations: &[(usize, usize)]) {
    for &(start, end) in operations.iter().rev() {
        if start < end && end <= s.len() {
            s.drain(start..end);
        }
    }
}


// Perform multiple deletions on a String
fn remove_slices_string(mut s: String, operations: &[(usize, usize)]) -> String {
    for &(start, end) in operations.iter().rev() {
        if start < end && end <= s.len() {
            let before = &s[..start];
            let after = &s[end..];
            s = format!("{}{}", before, after);
        }
    }
    s
}

// Generate a series of delete operations safely
fn generate_operations(len: usize, num_ops: usize, rng: &mut ThreadRng) -> Vec<(usize, usize)> {
    if len == 0 {
        // Return an empty vector of operations if the length is 0
        return Vec::new();
    }

    let mut ops = Vec::with_capacity(num_ops);
    for _ in 0..num_ops {
        let start = rng.gen_range(0..len);
        let end = rng.gen_range(start..=len);
        ops.push((start, end));
    }
    ops
}

fn bench_remove_slices_vec(c: &mut Criterion) {
    c.bench_function("remove_slices Vec<char>", |b| {
        b.iter_custom(|iters| {
            let mut rng = rand::thread_rng();
            let s = "this is a very long string meant to test the performance of slice removal. ".repeat(1000);
            let mut vec_chars: Vec<char> = s.chars().collect();

            let start_time = std::time::Instant::now();
            for _ in 0..iters {
                if vec_chars.len() > 0 {
                    let operations = generate_operations(vec_chars.len(), 1000, &mut rng);
                    vec_chars = remove_slices_vec(vec_chars, &operations);
                }
            }
            start_time.elapsed()
        });
    });
}

fn bench_remove_slices_vec_ref(c: &mut Criterion) {
    c.bench_function("remove_slices &mut Vec<char>", |b| {
        b.iter_custom(|iters| {
            let mut rng = rand::thread_rng();
            let s = "this is a very long string meant to test the performance of slice removal. ".repeat(1000);
            let mut vec_chars: Vec<char> = s.chars().collect();

            let start_time = std::time::Instant::now();
            for _ in 0..iters {
                if !vec_chars.is_empty() {
                    let operations = generate_operations(vec_chars.len(), 1000, &mut rng);
                    // Now the function modifies `vec_chars` directly
                    remove_slices_vec_ref(&mut vec_chars, &operations);
                }
            }
            start_time.elapsed()
        });
    });
}

fn bench_remove_slices_string(c: &mut Criterion) {
    c.bench_function("remove_slices String", |b| {
        b.iter_custom(|iters| {
            let mut rng = rand::thread_rng();
            let mut s = "this is a very long string meant to test the performance of slice removal. ".repeat(1000);

            let start_time = std::time::Instant::now();
            for _ in 0..iters {
                if s.len() > 0 {
                    let operations = generate_operations(s.len(), 1000, &mut rng);
                    s = remove_slices_string(s, &operations);
                }
            }
            start_time.elapsed()
        });
    });
}

criterion_group!(benches, bench_remove_slices_vec, bench_remove_slices_vec_ref, bench_remove_slices_string);
criterion_main!(benches);
