use ferrum_state::*;
use std::time::Instant;
fn main() {
    for n in [10_000usize, 20_000, 40_000] {
        let s = "a".repeat(n);
        let t = Instant::now();
        let r = find_cluster_break(&s, n, false);
        println!("n={} one backward find_cluster_break(from end) = {} in {:?}", n, r, t.elapsed());
    }
}
