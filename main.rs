// use rust_fundamental_3::learning_rust::Person;
// use rust_fundamental_3::learning_rust::Animal;
// use rust_fundamental_3::learning_rust::PersonId;
// use rust_fundamental_3::learning_rust::log_info;
// use rust_fundamental_3::learning_rust::log_info_2;
// use rust_fundamental_3::learning_rust::check_person_id;
// use rust_fundamental_3::learning_rust::Log;

// use rust_fundamental_3::learning_rust::*;

use rust_fundamental_3::learning_rust::{ Person, Animal, PersonId, Log, log_info, log_info_2, check_person_id };

fn main() {
  let mut person = Person::new();
  let animal = Animal(String::from("Pubby")); 
  person.change_age(25);
  // let my_agae = Box::new(43);

  let person_2 = Person::from("Anand".to_string(), "Raja".to_string(), 28, PersonId::Passport(360, 213, 843));

  person.display_info();
  person.alert_myself();
  println!("Id is {}", person.id);
  println!("Passport Id is {}", person_2.id);

  log_info_2(&person);
  log_info_2(&animal);
  log_info(person);
  // Person::alert_something();
  // Animal::alert_something();

  // check_person_id(person.id);
  check_person_id(person_2.id);

}