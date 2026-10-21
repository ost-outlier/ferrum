use ferrum_state::*;
use std::time::Instant;
fn main() {
    for n in [10_000usize, 20_000, 40_000] {
        let s = "a".repeat(n);
        let t = Instant::now();
        let c = count_column(&s, 4, None);
        let d1 = t.elapsed();
        let t = Instant::now();
        let mut k = 0; for _ in 0..1000 { k += find_cluster_break(&s, n / 2, true); }
        let d2 = t.elapsed();
        println!("n={} count_column={} in {:?}; 1000x find_cluster_break(mid) in {:?} ({})", n, c, d1, d2, k);
    }
}
