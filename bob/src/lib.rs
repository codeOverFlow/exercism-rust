pub fn reply(phrase: &str) -> String {
    let is_question = phrase.ends_with('?');
    let is_yelling = phrase
        .chars()
        .fold(true, |acc, c| acc && (c.is_uppercase() || !c.is_alphabetic()));
    println!("{}: {}", phrase, is_yelling);
    
    if phrase.is_empty() {
        String::from("Fine. Be that way!")
    }
    else if is_question {
        String::from("Sure.")
    }
    else if is_yelling {
        String::from("Whoa, chill out!")
    }
    else {
        String::from("Whatever.")
    }
}
