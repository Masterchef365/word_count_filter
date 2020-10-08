use std::cmp::Reverse;
use std::collections::HashMap;

fn main() {
    let counted_txt = std::fs::read_to_string("counted.txt").unwrap();
    let mut counted = parse_counted(&counted_txt);
    let counted_total: u64 = counted.iter().map(|(_, c)| c).sum();
    let freq_txt = std::fs::read_to_string("unigram_freq.csv").unwrap();
    let freq = parse_freq(&freq_txt);
    let freq_total: u64 = freq.values().sum();

    /*
    counted.sort_by_key(|(_, c)| Reverse(*c));
    for (word, count) in counted.iter().take(50) {
        println!("{}: {}", count, word);
    }
    */

    let common_denom = freq_total * counted_total;
    for (word, count) in &mut counted {
        if let Some(freq_count) = freq.get(word) {
            let lhs = *count * freq_total;
            let rhs = freq_count * counted_total;
            if lhs > rhs {
                *count = (lhs - rhs) / common_denom;
            } else {
                *count = 0;
            }
        }
    }

    //println!("=================");
    counted.sort_by_key(|(_, c)| Reverse(*c));
    for (idx, (word, _count)) in counted.iter().enumerate().take(250) {
        //println!("{}: {} ({})", idx, word, count);
        println!("{}: {}", idx, word);
    }
}

fn parse_counted<'a>(input_file: &'a str) -> Vec<(&'a str, u64)> {
    let mut map = Vec::new();
    for line in input_file.lines() {
        let mut iter = line.split(&['[', ',', '"', ']'][..]).skip(1);
        let count = iter.next().unwrap().parse().unwrap();
        let word = iter.skip(1).next().unwrap();
        if !word.contains(&[',', '\'', '!', '?', ')', '('][..])
            && word.len() > 1
            && !word.starts_with("said")
            && !word.chars().all(|c| c.is_digit(10))
        {
            map.push((word, count));
        }
    }
    map
}

fn parse_freq<'a>(input_file: &'a str) -> HashMap<&'a str, u64> {
    let mut map = HashMap::new();
    for line in input_file.lines().skip(1) {
        let mut iter = line.split(',');
        let word = iter.next().unwrap();
        let count = iter.next().unwrap().parse().unwrap();
        map.insert(word, count);
    }
    map
}
