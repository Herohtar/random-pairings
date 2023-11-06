use crate::person::Person;

use std::collections::VecDeque;

use rand::{
  seq::SliceRandom, Rng,
};

#[derive(Debug, Clone)]
pub struct PairingGenerator {
  people: Vec<Person>,
}

impl PairingGenerator {
  pub fn new() -> Self {
    PairingGenerator {
      people: Vec::new(),
    }
  }

  pub fn add_person(&mut self, name: &str) -> Result<(), String> {
    if self.people.iter().any(|p| p.name == name) {
      return Err(format!("Tried to add duplicate person: {}", name));
    }

    self.people.push(Person::new(name));

    Ok(())
  }

  pub fn add_couple(&mut self, first_name: &str, second_name: &str) -> Result<(), String> {
    if self.people.iter().any(|p| p.name == first_name) {
      return Err(format!("Tried to add duplicate person: {}", first_name));
    }
    if self.people.iter().any(|p| p.name == second_name) {
      return Err(format!("Tried to add duplicate person: {}", second_name));
    }

    self.people.push(Person::with_spouse(first_name, second_name));
    self.people.push(Person::with_spouse(second_name, first_name));

    Ok(())
  }

  pub fn generate_pairings<R>(&self, rng: &mut R) -> Vec<Person> where R: Rng + ?Sized {
    let mut unassigned_people = self.people.clone();
    let mut assigned_people: Vec<Person> = Vec::with_capacity(unassigned_people.len());
    let mut shuffled = unassigned_people.clone();
    shuffled.shuffle(rng);
    let mut shuffled = VecDeque::from(shuffled);

    while unassigned_people.len() > 0 {
      let mut person = unassigned_people.pop().unwrap();
      let mut choice = shuffled.pop_front().unwrap();
      let mut name = choice.name.clone();
      let mut retries = 0;
      while !person.is_valid_choice(&name) {
        if retries >= 2 {
          let swap_index = assigned_people.iter().position(|p| p.is_valid_choice(&name) && person.is_valid_choice(p.choice.as_ref().unwrap())).unwrap();
          name = match &assigned_people[swap_index].choice {
            Some(name) => name.clone(),
            None => panic!("Person with no choice on assigned list"),
          };
          assigned_people[swap_index].choice = Some(choice.name.clone());
          break;
        } else {
          shuffled.push_back(choice);
          choice = shuffled.pop_front().unwrap();
          name = choice.name.clone();
          retries += 1;
        }
      }
      person.choice = Some(name.clone());
      assigned_people.push(person);
    }

    assigned_people
  }
}
