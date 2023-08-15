use std::io;
use regex::Regex;

fn main() {
    println!("This program calculates the Fibonacci sequence of n");

    let mut n = String::from("00");

//    let mut n = String::new();
    let re1 = Regex::new(r"^[qQ]").unwrap();

//    let re2 = Regex::new(r"").unwrap();
//    while n != String::from("q") {
//    while n.as_bytes()[0] != (b'q') {
//    while &n[0] != '0'.to_string() {
//    while ![b'q', b'Q'].contains(&n.as_bytes()[0]) {
//    while !((&n.as_bytes().contains(&b'q')) | (&n.as_bytes().contains(&b'Q'))) {

//    while !re1.is_match(&n) && (&n).chars().count()>1 {
    while (&n).chars().count()>1 {
        n = "".to_string();
        println!("Enter n (q or Q to quit)");
        io::stdin()
           .read_line(&mut n)
           .expect("Failed to read input");
//        println!("n={n}");
        }
}
