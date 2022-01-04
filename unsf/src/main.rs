fn split(s: &str, delimiter: char) -> Option<(&str, &str)> {
    match s.find(delimiter) {
        Some(idx) => Some((&s[..idx], &s[idx + delimiter.len_utf8()..])),
        None => None,
    }
}

fn main() {
    assert_eq!(split("hello☺world", '☺'), Some(("hello", "world")));
}
