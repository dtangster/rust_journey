use std::collections::HashMap;

fn can_gen_str(target_str: &str, words: &[&str]) -> bool {
    if target_str.is_empty() {
        return true;
    }

    for &word in words {
        if word.len() > target_str.len() {
            continue;
        }

        let prefix = &target_str[..word.len()];
        let remaining = &target_str[word.len()..];

        if word == prefix && can_gen_str(remaining, words) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod can_gen_str_tests {
    use super::*;

    #[test]
    fn test_it_works() {
        assert!(true);
    }
}

fn how_gen_str<'a>(target_str: &'a str, words: &'a [&str]) -> Option<Vec<&'a str>> {
    if target_str.is_empty() {
        return Some(vec![]);
    }

    for &word in words {
        if word.len() > target_str.len() {
            continue;
        }

        let prefix = &target_str[..word.len()];

        if word != prefix {
            continue;
        }

        let remaining = &target_str[word.len()..];

        if let Some(rem_gen) = how_gen_str(remaining, words) {
            let mut how_accumulator = vec![word];

            how_accumulator.extend(rem_gen);
            return Some(how_accumulator);
        }
    }

    None
}

#[cfg(test)]
mod how_gen_str_tests {
    use super::*;

    #[test]
    fn test_it_works() {
        assert!(true);
    }
}

fn best_gen_str<'a>(target_str: &'a str, words: &'a [&str]) -> Option<Vec<&'a str>> {
    if target_str.is_empty() {
        return Some(vec![]);
    }

    let mut best: Option<Vec<&str>> = None;

    for &word in words {
        if word.len() > target_str.len() {
            continue;
        }

        let prefix = &target_str[..word.len()];

        if word != prefix {
            continue;
        }

        let remaining = &target_str[word.len()..];

        if let Some(suffix_gen) = best_gen_str(remaining, words) {
            let mut current_gen = vec![word];

            current_gen.extend(suffix_gen);
            match best {
                Some(ref best_gen) => {
                    if current_gen.len() < best_gen.len() {
                        best = Some(current_gen);
                    }
                }
                None => {
                    best = Some(current_gen);
                }
            }
        }
    }

    best
}

#[cfg(test)]
mod best_gen_str_tests {
    use super::*;

    #[test]
    fn test_it_works() {
        assert!(true);
    }
}

fn all_gen_str<'a>(target_str: &'a str, words: &'a [&str]) -> Vec<Vec<&'a str>> {
    if target_str.is_empty() {
        return vec![vec![]];
    }

    let mut results: Vec<Vec<&str>> = vec![];

    for &word in words {
        if word.len() > target_str.len() {
            continue;
        }

        let prefix = &target_str[..word.len()];

        if word != prefix {
            continue;
        }

        let remaining = &target_str[word.len()..];

        for suffix_gen in all_gen_str(remaining, words) {
            let mut current_gen = vec![word];

            current_gen.extend(suffix_gen);
            results.push(current_gen);
        }
    }

    results
}

#[cfg(test)]
mod all_gen_str_tests {
    use super::*;

    #[test]
    fn test_it_works() {
        assert!(true);
    }
}
