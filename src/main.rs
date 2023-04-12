// 이 프로그램은 텍스트 파일에서 단어의 빈도를 계산하고, 빈도별로 내림차순 정렬하여 출력합니다.
use std::collections::HashMap;
use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }
    let text = fs::read_to_string(&args[1]).expect("Failed to read file");
    let word_freqs = count_words(&text);
    let sorted_words = sort_by_frequency(word_freqs);
    print_word_frequencies(&sorted_words);
}
fn count_words(text: &str) -> HashMap<String, u32> {
    let mut word_freqs = HashMap::new();
    let special_chars = ".,";
    for word in text.split_whitespace() {
        let word = word.to_lowercase();
        let word = word
            .chars()
            .filter(|c| !special_chars.contains(*c))
            .collect::<String>(); // 특수 문자를 제거합니다.
        let count = word_freqs.entry(word).or_insert(0);
        *count += 1;
    }
    word_freqs
}
fn sort_by_frequency(word_freqs: HashMap<String, u32>) -> Vec<(String, u32)> {
    let mut sorted_words: Vec<(String, u32)> = word_freqs.into_iter().collect();
    sorted_words.sort_by(|a, b| b.1.cmp(&a.1));
    sorted_words
}
fn print_word_frequencies(sorted_words: &[(String, u32)]) {
    println!("Word frequencies:");
    for (word, freq) in sorted_words {
        println!("{: <20} {}", word, freq);
    }
}
