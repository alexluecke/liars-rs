extern crate rand;
use rand::{thread_rng, Rng};
use std::fmt;

enum Type {
    Liar,
    Honest,
}

struct Person {
    integrity: Type,
}

impl fmt::Debug for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.integrity {
            Type::Liar => write!(f, "Liar"),
            _ => write!(f, "Honest")
        }
    }
}

fn ask_if_liar(x: &Person, y: &Person) -> bool {
    match (*x).integrity {
        Type::Honest => match (*y).integrity {
            Type::Liar => true,
            _ => false,
        },
        _ => generate_bool()
    }
}

fn generate_bool() -> bool {
    *thread_rng().choose(&[true, false]).unwrap()
}

fn main() {
    let num_people = 9;

    let mut max_liars: isize = match num_people%2 {
        0 => num_people/2 - 1,
        _ => num_people/2
    };
    let max = max_liars;

    let mut set_a: Vec<Person> = Vec::new();
    let mut set_b: Vec<Person> = Vec::new();

    let mut people: Vec<Person> = (0..num_people)
        .map(|_| match generate_bool() {
            true => Person { integrity: Type::Honest },
            _ => {
                max_liars -= 1;
                match max_liars > 0 {
                    true => Person { integrity: Type::Liar },
                    _ => Person { integrity: Type::Honest }
                }
            },
        })
    .collect();


    println!("People: ");
    for person in people.iter().rev() {
        match (*person).integrity {
            Type::Liar => print!("Liar, "),
            _ => print!("Honest, "),
        }
    }
    println!("\n");

    let mut count = 0;
    while people.len() > 0 {

        let x = people.pop();
        let y = if set_a.len() > 0 {
            set_a.pop()
        } else {
            people.pop()
        };

        match (x, y) {
            (Some(x), Some(y)) => {
                if ask_if_liar(&x, &y) {
                    println!("({:?}) says ({:?}) is liar => put in set B.", x, y);
                    set_b.push(x);
                    set_b.push(y);
                } else {
                    println!("({:?}) says ({:?}) is honest => put in set A.", x, y);
                    set_a.push(x); // push x first!
                    set_a.push(y);
                }
            },
            (Some(x), _) => println!("DM was {:?}", x),
            (_, _) => println!("Oh no.")
        }

        count += 1;
        println!("round {}, set_a.let(): {}, max: {}", count, set_a.len(), max);

        if set_a.len() as isize > max {
            println!("\nSet A grew too big");
            match set_a.pop() {
                Some(x) => match x.integrity {
                    Type::Honest => println!("DM was honest"),
                    _ => println!("DM was a liar"),
                },
                None => println!("Oh boy, this should not have happened."),
            }
            break;
        }

        if set_b.len() as isize >= 2*max {
            println!("\nSet B grew too big");
            if set_a.len() > 0 {
                match set_a.pop() {
                    Some(x) => match x.integrity {
                        Type::Honest => println!("DM was honest"),
                        _ => println!("DM was a liar"),
                    },
                    None => println!("Oh boy, this should not have happened."),
                }
            } else {
                match people.pop() {
                    Some(x) => match x.integrity {
                        Type::Honest => println!("DM was honest"),
                        _ => println!("dm was a liar"),
                    },
                    None => println!("Oh boy, this should not have happened."),
                }
            }
            break;
        }
    }

    println!("\n--------------------------------------------------");
    println!("Set A");
    for person in set_a.iter() {
        match (*person).integrity {
            Type::Liar => println!("Liar"),
            _ => println!("Honest"),
        }
    }

    println!("\n--------------------------------------------------");
    println!("Set B");
    for person in set_b.iter() {
        match (*person).integrity {
            Type::Liar => println!("Liar"),
            _ => println!("Honest"),
        }
    }

}
