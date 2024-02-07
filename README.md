# VecCharOrStringSlicing

# Slice Removal Performance Benchmarks

This suite of benchmarks evaluates the performance of different methods for removing slices from strings and vectors in Rust. It uses Criterion.rs for precise measurement.

## Benchmarked Functions

- `remove_slices_vec`: Removes multiple slices from a `Vec<char>` by using the `drain` method.
- `remove_slices_vec_ref`: Similar to `remove_slices_vec`, but operates on a mutable reference to a `Vec<char>` instead of taking ownership.
- `remove_slices_string`: Removes multiple slices from a `String`, constructing a new string for each operation by concatenating the parts before and after the slice to be removed.

## Generating Delete Operations

The benchmarks simulate real-world usage by performing a series of delete operations on a string or vector. These operations are generated by `generate_operations`, which creates a list of start and end indices for each delete operation. The number of operations and the size of the target data structure are parameters to this function.

## Benchmark Scenarios

Each benchmark measures the time it takes to perform 1000 delete operations on a long string (repeated 1000 times to increase the size and make the benchmark more demanding). This process is repeated for a specified number of iterations to ensure statistical significance of the results.

- **`bench_remove_slices_vec`**: Measures performance for a `Vec<char>`.
- **`bench_remove_slices_vec_ref`**: Measures performance for a mutable reference to a `Vec<char>`.
- **`bench_remove_slices_string`**: Measures performance for a `String`.

## Running the Benchmarks

To run the benchmarks, ensure you have Criterion.rs added to your `Cargo.toml` and the Rust toolchain installed. Then, execute the following command in your terminal:

```bash
cargo bench
```


This command compiles the benchmarks and runs them, outputting the results to your terminal and generating detailed reports in `target/criterion`.

## Analyzing Results

Criterion.rs provides detailed reports that include graphs and statistical analysis. These reports can help identify performance regressions and improvements over time. Access the reports in the `target/criterion` directory after running the benchmarks.

