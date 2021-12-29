mod another_lib; // importing another library inside current mod

use another_lib::another_mod;

fn outsider_fn() {
  another_lib::another_mod::another_fn(); // full path
  another_mod::another_fn(); // as we've used "use" keyword as alias
  crate::another_mod::another_fn();
  println!("Outsider Function");
}

pub mod learning_rust {

  use std::fmt;

  mod top_level {
    pub fn hi_there() {
      println!("Hi There");
    }

    pub mod low_level {
      pub fn hello_world() {
        println!("Hello World!");
      }
    }
  }

  pub trait Log {
    fn display_info(&self);

    // fn alert_something() {
    //   println!("Default implementation!");
    // }

    fn alert_myself(&self) {
      println!("hello Rust!");
    }
  }


  pub enum PersonId {
    Passport(u32, u32, u32),
    IdentityCard(u32)
  }

  // Implementing Display
  
  impl fmt::Display for PersonId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
          PersonId::IdentityCard(x) => {
            write!(f, "{}", x)
          },
          PersonId::Passport(x, y, z) => {
            write!(f, "{} {} {}", x, y, z)
          }
        }
    }
  }

  pub struct Person {
    name: String,
    last_name: String,
    age: u32,
    pub id: PersonId
  }

  impl Log for Person {
    fn display_info(&self) {
      println!("{} {} {} {}", self.name, self.last_name, self.age, self.id);
      // calling fn inside module : absolute import
      // crate points to src/lib.rs or src/main.rs 
      crate::learning_rust::top_level::hi_there();
      // relative import
      top_level::hi_there();

      top_level::low_level::hello_world();

      crate::outsider_fn();
      // to access outside of current module (one level up from current module) super::super means 2 level up
      super::outsider_fn();
    }
  }

  impl Log for Animal {
    fn display_info(&self) {
      println!("Animal name is {}", self.0);
    }
  }

  impl Person {
    pub fn new() -> Person {
      Person {
        name: "first name".to_string(),
        last_name: "last name".to_string(),
        age: 0,
        id: PersonId::IdentityCard(413761)
      }
    }

    pub fn from(name: String, last_name: String, age: u32, id:PersonId) -> Person {
      Person {
        name,
        last_name,
        age,
        id
      }
    }

    pub fn change_age(&mut self ,new_age: u32) {
      self.age = new_age;
    }

  }

  pub struct Animal(pub String);

  pub fn check_person_id(id: PersonId) {

    if let PersonId::IdentityCard(num) = id {
      println!("ID is matching {}", num);
    } else {
      println!("No match!");
    }

    let result = match id {
      PersonId::IdentityCard(x) => {
        x
      },
      PersonId::Passport(_x, y, _z) => {
        y
      }
    };

    let animal = Animal(String::from("Cat")); 
    animal.display_info();
    let Animal(animal_type) = animal;

    println!("Animal is {}", animal_type);
    println!("result {}", result);

  }

  pub fn log_info(val: impl Log) {
    val.alert_myself();
  }

  pub fn log_info_2(val: &dyn Log) {
    val.alert_myself();
  }
}