fn main() {
    let vowels = "aeiou";
    let word = "First";
    let c1 = word.chars().next().unwrap().to_ascii_lowercase();
    if vowels.contains(c1) {
        let mut pig = String::from(word);
        pig.push_str("-hay");
        dbg!(pig);
    } else {
        let mut pig = String::from(&word[1..]);
        pig.push_str(&format!("-{c1}ay"));
        dbg!(pig);
    }
}
