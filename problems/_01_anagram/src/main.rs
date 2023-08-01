use banner::print_banner;
use std::collections::HashMap;

fn main() {
    print_banner();

    let words = vec![
        "the".to_string(),
        "teh".to_string(),
        "hte".to_string(),
        "apple".to_string(),
        "appel".to_string(),
    ];

    let groups = group_by(words);

    println!("group_by {:?}", groups);
}

fn group_by(words: Vec<String>) -> Vec<Vec<String>> {
    let mut word_map = HashMap::new();

    let mut char_freq = vec![0; 26];

    for word in words {
        for c in word.to_lowercase().chars() {
            char_freq[(c as u32 - 'a' as u32) as usize] += 1;
        }

        let key = char_freq
            .into_iter()
            .map(|c| c.to_string())
            .collect::<String>();

        word_map.entry(key).or_insert(Vec::new()).push(word);

        char_freq = vec![0; 26];
    }

    word_map.into_iter().map(|(key, value)| value).collect()
}
