use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut is_anagram = true;
    let mut return_values: HashSet<&'a str> = HashSet::new();
    let mut word_vec: Vec<_> = word.to_ascii_lowercase().chars().collect();
    word_vec.sort_by(|a, b| a.cmp(b));
    println!("{:?}", word_vec);

    for anagram in possible_anagrams {
        let mut anagram_vec: Vec<_> = anagram.to_ascii_lowercase().chars().collect();
        anagram_vec.sort_by(|a, b| a.cmp(b));
        println!("{:?}", anagram_vec);
        

        if word_vec == anagram_vec {
            return_values.insert(anagram);
        }
        is_anagram = true;
    }

    return_values
}
