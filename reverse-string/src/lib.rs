use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let g = UnicodeSegmentation::graphemes(input, true).collect::<String>();
    println!("Write a function to reverse {}", g);

    let rev = g.chars().rev().collect::<String>();
    rev
}


