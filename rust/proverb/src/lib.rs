pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb = String::new();
    for (index, item) in list.iter().enumerate() {
        if index < list.len() - 1 {
            proverb.push_str(&format!("For want of a {} the {} was lost.\n", item, list[index+1]));
        } else {
            proverb.push_str(&format!("And all for the want of a {}.", list[0]));
        }
    }
    proverb
}

use std::iter::once;

// using windows - gives a kind of bundled up view on the data

pub fn build_proverb_refactor(list: &[&str]) -> String {
    let last = format!("And all for the want of a {}.", &list[0]);

    list
        .windows(2)
        .map(|w|build_verse(w[0], w[1]))
        .chain(once(last)) // chain takes two iterators and creates a new one with both in sequence
        // here we create a new iterator with once (which makes an iterator that does one this exactly once)
        .collect::<Vec<String>>()
        .join("\n")
}

fn build_verse(left: &str, right: &str) -> String {
    format!("For want of a {} the {} was lost.", left, right)
}

// another way to iter over index => for index in 0..(list.len() - 1)
