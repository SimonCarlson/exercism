use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut return_values: HashSet<&'a str> = HashSet::new();
    let mut word_vec: Vec<_> = word.chars().collect();
    word_vec[0].make_ascii_lowercase();
    word_vec.sort_by(|a, b| a.cmp(b));
    println!("{:?}", word_vec);

    for anagram in possible_anagrams {
        let mut anagram_vec: Vec<_> = anagram.chars().collect();
        anagram_vec[0].make_ascii_lowercase();
        anagram_vec.sort_by(|a, b| a.cmp(b));
        println!("{:?}", anagram_vec);
        
        if word_vec == anagram_vec && &word != anagram {
            return_values.insert(anagram);
        }
    }

    return_values
}
