
use url::Url;
use std::collections::HashMap;
use regex::Regex;

pub fn render_named(src: &str, vals: &HashMap<&str, &str>) -> String {
    let mut parts: Vec<&str> = vec![];
    let regex = Regex::new(r"\{([\w]+)\}").unwrap();

    let matches: Vec<(usize,usize)> = regex
        .find_iter(src)
        .map(|m| (m.start(), m.end()))
        .collect();

    println!("{:?}", matches);

    let mut offset = 0usize;

    for (start, end) in matches.iter() {
        parts.push(&src[offset..*start]);

        // argument name with braces
        let arg = &src[*start..*end];
        // just the argument name
        let arg_name = &src[*start+1..*end-1];

        println!("arg name: {}", arg_name);

        // if value passed for argument then append it, otherwise append original argument
        // name with braces
        match vals.get(arg_name) {
            Some(s) => parts.push(s),
            _ => parts.push(arg),
        }

        offset = *end;
    }

    if offset < src.len() {
        parts.push(&src[offset..src.len()]);
    }

    parts.join("")
}

#[test]
fn test_url_parse() {
    let mut url: Url = Url::parse("http://localhost/{name}/{id}/").unwrap();
    for seg in url.path_segments_mut() {
        println!("{:?}", seg);
    }

    let path = url.path();
    println!("path: {}", path);
}

#[test]
fn test_replace() {
    let path = "/path1/{name}/hello/{id}/resource";
    let mut data = HashMap::new();
    data.insert("name","lyf");
    data.insert("id","123");

    let path = render_named(path, &data);
    println!("{}", path)
}