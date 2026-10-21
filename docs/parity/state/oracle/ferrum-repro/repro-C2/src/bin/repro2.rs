use ferrum_state::*;
fn main() {
    let mk = |v: Vec<i64>| ChangeDesc::create(v);
    println!("R2q compose_desc with empty wrong-length => {}", mk(vec![2, -1, 0, 2]).compose_desc(&mk(vec![30, -1])));
    println!("R2r compose empty-this wrong length => {}", ChangeSet::empty(3).compose(&ChangeSet::of_spec(ChangeSpec::insert(0, "q"), 9)));
    println!("R1p insert Text::empty => {}", ChangeSet::of_spec(ChangeSpec::insert(2, Text::empty()), 5));
    let d = Text::from("ab");
    println!("R12 apply empty returns equal doc => {}", ChangeSet::empty(2).apply(&d) == d);
}
