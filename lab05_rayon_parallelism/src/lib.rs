use rayon::prelude::*;

pub fn process_data_parallel(data: &[u32]) -> Vec<u32> {
    data.par_iter()
        .map(|&x| {
            // Complex bit-shift operation (Xorshift step)
            let mut y = x;
            y ^= y << 13;
            y ^= y >> 17;
            y ^= y << 5;
            y
        })
        .collect()
}

pub fn process_data_serial(data: &[u32]) -> Vec<u32> {
    data.iter()
        .map(|&x| {
            let mut y = x;
            y ^= y << 13;
            y ^= y >> 17;
            y ^= y << 5;
            y
        })
        .collect()
}
