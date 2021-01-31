
use std::collections::HashMap;
use regex::Regex;

pub fn replace_named(src: &str, params: &HashMap<&str, &str>) -> String {
    let mut parts: Vec<&str> = vec![];
    let regex = Regex::new(r"\{([\w]+)\}").unwrap();

    let matches: Vec<(usize,usize)> = regex
        .find_iter(src)
        .map(|m| (m.start(), m.end()))
        .collect();

    let mut offset = 0usize;

    for (start, end) in matches.iter() {
        parts.push(&src[offset..*start]);
        let arg = &src[*start..*end];
        let arg_name = &src[*start+1..*end-1];

        match params.get(arg_name) {
            Some(s) => parts.push(s),
            None => parts.push(arg),
        }

        offset = *end;
    }

    if offset < src.len() {
        parts.push(&src[offset..src.len()]);
    }

    parts.join("")
}

#[test]
fn test_replace() {
    let mut data = HashMap::new();
    data.insert("name","lyf");
    data.insert("id","123");

    assert_eq!(replace_named("/path1/{name}/hello/{id}/res", &data), "/path1/lyf/hello/123/res");

    assert_eq!(replace_named("/{name}/{id}", &data), "/lyf/123");
    assert_eq!(replace_named("/{name}/{id}/", &data), "/lyf/123/");
    assert_eq!(replace_named("{name}/{id}/", &data), "lyf/123/");
    assert_eq!(replace_named("{name2}/{id}/", &data), "{name2}/123/");
    assert_eq!(replace_named("/path/res", &data), "/path/res");
}