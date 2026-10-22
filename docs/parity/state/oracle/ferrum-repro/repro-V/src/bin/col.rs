use ferrum_state::*;
use std::time::Instant;
fn main() {
    let ns: Vec<usize> = std::env::args().skip(1).map(|a| a.parse().unwrap()).collect();
    for n in ns {
        let s = "a".repeat(n);
        let t = Instant::now();
        let c = count_column(&s, 4, None);
        println!("n={} count_column={} in {:?}", n, c, t.elapsed());
    }
}
