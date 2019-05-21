use std::str::FromStr;

#[derive(Debug)]
struct People {
    id: i32,
    idf: f32,
}

#[derive(Debug)]
struct PeopleParseError {
    num: i32,
}

impl FromStr for People {
    type Err = PeopleParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s
            .trim_matches(|p| p == '(' || p == ')')
            .split(',')
            .collect();

        println!("{:?}", parts);

        let id = match parts[0].parse::<i32>() {
            Ok(id) => id,
            Err(_) => return Err(PeopleParseError { num: 1 }),
        };

        let idf = match parts[1].parse::<f32>() {
            Ok(id) => id,
            Err(_) => return Err(PeopleParseError { num: 2 }),
        };

        Ok(People { id: id, idf: idf })
    }
}

fn main() {
    let people: Option<People> = match FromStr::from_str("(A,3)") {
        Ok(p) => Some(p),
        Err(e) => {
            println!("error in convert {:?}", e);
            None
        }
    };

    if people.is_some() {
        println!("from_str -> {:?}", people.unwrap());
    }
}
