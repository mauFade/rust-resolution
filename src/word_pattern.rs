use std::collections::HashMap;

pub(crate) fn word_pattern(pattern: &str, str_val: &str) -> bool {
    let mut pattern_map = HashMap::new();
    let mut str_map = HashMap::new();

    let pattern_chars: Vec<char> = pattern.chars().collect();
    let str_words: Vec<&str> = str_val.split_whitespace().collect();

    if pattern_chars.len() != str_words.len() {
        return false;
    }

    for (p, s) in pattern_chars.iter().zip(str_words.iter()) {
        match (pattern_map.get(p), str_map.get(s)) {
            (Some(&p_val), Some(&s_val)) => {
                if p_val != s_val {
                    return false;
                }
            }
            (None, None) => {
                pattern_map.insert(*p, str_val);
                str_map.insert(*s, pattern);
            }
            _ => return false,
        }
    }

    true
}
