use Difference;
use split::strsplit;

// merges the changes from two strings, given a common substring
pub fn merge(orig: &str, edit: &str, common: &str, separator: &str) -> Vec<Difference> {
    let mut ret = Vec::new();

    let lv = strsplit(orig, separator);
    let rv = strsplit(edit, separator);
    let cv = strsplit(common, separator);

    let mut l = lv.iter().peekable();
    let mut r = rv.iter().peekable();
    let mut c = cv.iter().peekable();

    while l.peek().is_some() || r.peek().is_some() {
        println!("L={:?},R={:?},C={:?}", l.peek(), r.peek(), c.peek());

        if l.peek().is_some() && l.peek() == c.peek() && r.peek() == c.peek() {
            ret.push(Difference::Same(l.next().unwrap().to_string()));
            r.next();
            c.next();
        }

        if l.peek().is_some() && l.peek() != c.peek() {
            ret.push(Difference::Rem(l.next().unwrap().to_string()));
        }

        if r.peek().is_some() && r.peek() != c.peek() {
            ret.push(Difference::Add(r.next().unwrap().to_string()));
        }
    }

    ret
}

#[test]
fn test_merge_1() {
    assert_eq!(
        merge("testa", "tost", "tst", ""),
        vec![
            Difference::Same("t".to_string()),
            Difference::Rem("e".to_string()),
            Difference::Add("o".to_string()),
            Difference::Same("s".to_string()),
            Difference::Same("t".to_string()),
            Difference::Rem("a".to_string()),
        ]
    );
}

#[test]
fn test_merge_2() {
    assert_eq!(
        merge("", "a", "", ""),
        vec![Difference::Add("a".to_string())]
    );
}

#[test]
fn test_merge_3() {
    assert_eq!(
        merge("a\nb", "a\n\nb", "a\nb", "\n"),
        vec![
            Difference::Same("a".to_string()),
            Difference::Add("".to_string()),
            Difference::Same("b".to_string()),
        ]
    );
}

#[test]
fn test_merge_4() {
    assert_eq!(
        merge("a\n", "c\n", "\n", "\n"),
        vec![
            Difference::Rem("a".to_string()),
            Difference::Add("c".to_string()),
            Difference::Same("".to_string()), // new line
        ]
    );
}
