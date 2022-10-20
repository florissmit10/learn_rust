use std::collections::HashMap;
use std::io;
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
        static ref RE_ADD: Regex = Regex::new(r"Add ([a-zA-Z]*) to ([a-zA-Z]*)").unwrap();
        static ref RE_LIST: Regex = Regex::new(r"List.*").unwrap();
}

fn main() {
    let mut dept_employees: HashMap<String, Vec<String>>=  HashMap::new();
    loop {

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        match match_input(&guess) {

            Some((name, dept)) => {
                dept_employees
                    .entry(dept)
                    .or_insert(Vec::new())
                    .push(name);
                println!("Added employees: {dept_employees:?}" )
            }
            _ => {  }
        }

        if RE_LIST.is_match(&guess){
            println!("employees: {dept_employees:?}" )
        }
    }
}

fn match_input(input: &str) -> Option<(String, String)> {
    let caps = match RE_ADD.captures(input)   {
        Some(s) => s,
        None => {return None}
    };

    let name = caps.get(1).map_or("", |m| m.as_str());
    let dept = caps.get(2).map_or("", |m| m.as_str());
    println!("{name} to {dept}");
    return if name.is_empty() || dept.is_empty()
    {
        println!("Return None");
        None
    } else {
        Some((
            String::from(name),
            String::from(dept)
        ))    };
}
