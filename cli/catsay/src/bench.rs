#![feature(test)]
extern crate test;
#[cfg(test)]
mod tests {
    #[bench]
    fn bench_main_1000_times() {
        for _ in 0..1000 {
            main::main()
        }
    }
}
