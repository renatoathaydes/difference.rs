
// strsplit is like `s.split(split)`, except that if `split` is "", it
// trims the leading and trailing empty elements, since the `lcs`
// logic won't handle those properly.
pub fn strsplit<'a>(s: &'a str, split: &str) -> Vec<&'a str> {
    let mut si = s.split(split);
    if split == "" {
        si.next();
    }
    let mut v: Vec<&str> = si.collect();
    if split == "" {
        v.pop();
    }
    v
}
