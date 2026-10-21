// Reads cases.txt (id \t hex code points) and prints Ferrum results in the same format as cm6-results.txt.
// Usage: charcol <dir>
use ferrum_state::*;
use std::fs;

fn main() {
    let dir = std::env::args().nth(1).unwrap();
    let data = fs::read_to_string(format!("{}/cases.txt", dir)).unwrap();
    let mut out = String::new();
    for line in data.lines() {
        let mut parts = line.splitn(2, '\t');
        let id = parts.next().unwrap();
        let hex = parts.next().unwrap_or("");
        let s: String = hex
            .split_whitespace()
            .map(|h| char::from_u32(u32::from_str_radix(h, 16).unwrap()).unwrap())
            .collect();
        let n = s.chars().count();
        let mut fwd = vec![];
        let mut bwd = vec![];
        let mut cc = vec![];
        for i in 0..=n {
            fwd.push(find_cluster_break(&s, i, true).to_string());
            bwd.push(find_cluster_break(&s, i, false).to_string());
            cc.push(count_column(&s, 4, Some(i)).to_string());
        }
        let mut fc = vec![];
        let mut fcs = vec![];
        for col in 0..=(n * 4 + 2) {
            fc.push(find_column(&s, col, 4, false).unwrap().to_string());
            fcs.push(match find_column(&s, col, 4, true) {
                Some(v) => v.to_string(),
                None => "none".to_string(),
            });
        }
        out.push_str(&format!(
            "{}\tfwd:{}\tbwd:{}\tcc:{}\tccfull:{}\tfc:{}\tfcs:{}\n",
            id,
            fwd.join(","),
            bwd.join(","),
            cc.join(","),
            count_column(&s, 4, None),
            fc.join(","),
            fcs.join(",")
        ));
    }
    fs::write(format!("{}/ferrum-results.txt", dir), out).unwrap();
}
