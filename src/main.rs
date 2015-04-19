extern crate rand;
use rand::{thread_rng, Rng};
use std::num::Float;

enum Person {
    Liar,
    Honest,
}

fn ask_if_liar(x: &Person, y: &Person) -> bool {
    match *x {
        Person::Honest => match *y {
                Person::Liar => true,
                _ => false,
            },
        _ => generate_bool()
    }
}

fn generate_random_person() -> Person {
    let mut rng = thread_rng();
    match *rng.choose(&[0, 1]).unwrap() {
        0 => Person::Honest,
        _ => Person::Liar,
    }
}

fn generate_bool() -> bool {
    *thread_rng().choose(&[true, false]).unwrap()
}

fn main() {
    let num_people = 9;
    let mut max_liars = num_people/2;
    let mut set_a: Vec<Person> = Vec::new();
    let mut set_b: Vec<Person> = Vec::new();
    let mut d: &Person;

    let mut people: Vec<Person> = (0..num_people)
        .map(|_| match generate_random_person() {
            Person::Honest => Person::Honest,
            _ => {
                    max_liars -= 1;
                    match max_liars > 0 {
                        true => Person::Liar,
                        _ => Person::Honest
                    }
                },
        })
        .collect();


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
                   set_b.push(x);
                   set_b.push(y);
               } else {
                   set_a.push(x); // push x first!
                   set_a.push(y);
               }
           }
           (Some(x), _) => println!("One more"),
           _ => println!("done")
       }
   }

   for person in set_a.iter() {
       match *person {
           Person::Liar => println!("Liar"),
           _ => println!("Honest"),
       }
   }

   println!("--------------------------------------------------");

   for person in set_b.iter() {
       match *person {
           Person::Liar => println!("Liar"),
           _ => println!("Honest"),
       }
   }

    //for person in people.iter() {
        //match *person {
            //Person::Liar => println!("Liar"),
            //_ => println!("Honest"),
        //}
    //}
}
