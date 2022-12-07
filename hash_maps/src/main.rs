use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);

    let team_name = String::from("Blue");
    println!(
        "Team {team_name} has {:?} points",
        scores.get(&team_name).copied().unwrap_or(0)
    );

    scores.entry(String::from("Pink")).or_insert(100);
    scores.entry(String::from("Pink")).or_insert(20);

    for tuple in &scores {
        println!("Team {} has {} points", tuple.0, tuple.1);
    }
    println!("{:?}", scores);

    let text = "Hello world wonderful world";
    let mut wc: HashMap<&str, i32> = HashMap::new();
    for word in text.split_whitespace() {
        let count = wc.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", wc);
}
