use ferrum_state::*;
fn main() {
    let st = EditorState::create(EditorStateConfig::default());
    let cat = st.char_categorizer(0);
    let mut out = String::with_capacity(0x110000);
    for c in 0u32..0x110000 {
        match char::from_u32(c) {
            None => out.push('-'),
            Some(ch) => {
                let mut b = [0u8; 4];
                let a = cat(ch.encode_utf8(&mut b));
                assert_eq!(a, categorize_char(ch));
                out.push(match a { CharCategory::Word => 'W', CharCategory::Space => 'S', CharCategory::Other => 'O' })
            }
        }
    }
    std::fs::write(std::env::args().nth(1).unwrap(), out).unwrap();
}
