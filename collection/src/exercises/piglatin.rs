use std::io;

pub fn piglatin() {
    println!("Please input a string of words.");
    let mut sent = String::new();

    io::stdin()
       .read_line(&mut sent)
       .expect("Failed to read line.");

    let mut words = sent.split_whitespace();

    let mut pigsent = String::new();

    for word in words.by_ref() {
        let mut trimmed = word.trim_end().to_string();

        let firstletter: char = word.chars().nth(0).expect("Cannot find first letter.");

        if firstletter == 'a' || firstletter == 'e' || firstletter == 'i' || firstletter == 'o' || firstletter == 'u' {
            let pigword = format!(" {trimmed}-hay ");
            pigsent.push_str(&pigword);
        } else {
            trimmed.remove(0);
            let pigword = format!(" {trimmed}-{firstletter}ay ");
            pigsent.push_str(&pigword);
        }
    }
    println!("The pig latin of given sentence is: {pigsent}")
}
