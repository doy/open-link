use std::io::prelude::*;

fn read_stdin() -> String {
    let mut input = String::new();
    let stdin = std::io::stdin();
    let mut stdin_handle = stdin.lock();
    stdin_handle.read_to_string(&mut input).unwrap();
    input
}

fn main() {
    let input = read_stdin();
    let finder = linkify::LinkFinder::new();
    let links: Vec<_> = finder.links(&input).map(|l| l.as_str()).collect();
    if !links.is_empty() {
        open::that(links[links.len() - 1]).unwrap();
    }
}
