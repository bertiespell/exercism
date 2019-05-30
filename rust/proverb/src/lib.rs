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
