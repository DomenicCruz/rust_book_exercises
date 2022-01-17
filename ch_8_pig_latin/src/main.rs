use std::io;
use std::collections::HashMap;

fn main() {
    println!("Convert text to pig latin\nEnter text:");
    
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("failed to read from stdin");

   println!("Pig Latin:\n{}", to_pig_latin(&text));

}


fn to_pig_latin(text: &String) -> String {
    //Hash map with vowels
    let vowels_list = ['a', 'e', 'i', 'o', 'y', 'u'].into_iter();
    let mut vowels = HashMap::new();
    vowels_list.for_each(|item| {
        vowels.insert(item, true);
    });


    let mut pig_latin = String::new();
    let words = text.split_whitespace();
    for word in words {
        //println!("first char {:?}", word.chars().collect::<Vec<_>>()[0]);

        let mut first_char = '_';
        let mut other_chars_str = String::new();
        let mut first = true;
        for ch in word.to_string().chars() {
            if first {
                first = false;
                first_char = ch;
                //first_char.push(ch);

            } else {
                other_chars_str.push(ch);
            }
        }
        //if first char is a vowel add '-fay' to the end
        if vowels.contains_key(&first_char) {
            pig_latin.push_str(&word);
            pig_latin.push_str("-hay");
        } else {
            //else add all chars beside first to the new string 
            pig_latin.push_str(&other_chars_str);
            //add -ay + first char
            pig_latin.push_str(&format!("-{}ay", first_char));
        }
        pig_latin.push_str(" ");
    }
    pig_latin
}
