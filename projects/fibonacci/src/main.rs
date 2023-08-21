use std::io;
use
{
    once_cell::sync::Lazy,
    regex::Regex,
};
use std::{thread, time};

fn input() -> String
{
    let mut n = String::new();
    println!("Enter n (q or Q to quit)");
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read input");
    return n.trim().to_string();
}

fn fibonacci(n: u8) -> u128
{
    if n<2
    {
        return n as u128;
    }
    let mut n0: u128 = 0;
    let mut n1: u128 = 1;
    let mut i: u8 = 2;
    let mut fib: u128 = 0;
    while i<=n
    {
        fib = n0 + n1;
        n0 = n1;
        n1 = fib;
        i += 1;
    }
    fib
}

fn welcome()
{
    clearscreen::clear().expect("Failed to clear the screen");
    println!("This program calculates the Fibonacci sequence of n");
}

fn main()
{
    static RE1: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[qQ]").unwrap());
    static RE2: Lazy<Regex> = Lazy::new(|| Regex::new(r"[^0-9]").unwrap());
    static RE3: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(\s*$)").unwrap());
    welcome();
    let mut n = input();
    while !(RE1.is_match(&n) && (&n).chars().count()==1)
    {
        while RE2.is_match(&n) && !(RE1.is_match(&n) && (&n).chars().count()==1)
        {
            println!("\"{}\" is not a natural number!",&n);
            thread::sleep(time::Duration::from_millis(800));
            welcome();
            n = input();
        }
        if !(RE1.is_match(&n) && (&n).chars().count()==1)
        {
            if RE3.is_match(&n)
            {
                println!("Empty value entered!");
                thread::sleep(time::Duration::from_millis(800));
            }
            else
            {
                let n_as_int = n.parse::<u8>().unwrap();
                if n_as_int>186
                {
                    println!("Cannot calculate Fibonacci's sequence for n greater than 186");
                    thread::sleep(time::Duration::from_millis(1800));
                }
                else
                {
                    println!("The Fibonacci number for {n} is {}",fibonacci(n_as_int));
                    thread::sleep(time::Duration::from_millis(1800));
                }
            }
            welcome();
            n = input();
        }
    }
}
