use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Person {
  pub name: String,
  pub spouse: Option<String>,
  pub choice: Option<String>,
}

impl Person {
  pub fn new(name: &str) -> Self {
    Person {
      name: name.to_string(),
      spouse: None,
      choice: None,
    }
  }

  pub fn with_spouse(name: &str, spouse: &str) -> Self {
    Person {
      name: name.to_string(),
      spouse: Some(spouse.to_string()),
      choice: None,
    }
  }

  pub fn is_valid_choice(&self, name: &str) -> bool {
    let mut valid = true;

    if self.name == name {
      valid = false;
    } else if let Some(spouse) = self.spouse.as_ref() {
      valid = spouse != name;
    }

    valid
  }
}

impl fmt::Display for Person {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let choice = match &self.choice {
      Some(choice) => choice.clone(),
      None => "None".to_string(),
    };
    write!(f, "{} is buying a gift for {}", self.name, choice)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new_works() {
    let name = "Bob".to_string();
    assert_eq!(Person::new(&name), Person {
      name: name,
      spouse: None,
      choice: None,
    });
  }

  #[test]
  fn with_spouse_works() {
    let name = "Bob".to_string();
    let spouse = "Alice".to_string();
    assert_eq!(Person::with_spouse(&name, &spouse), Person {
      name: name,
      spouse: Some(spouse),
      choice: None,
    });
  }

  #[test]
  fn valid_choice_without_spouse_is_correct() {
    let name = "Bob".to_string();
    let choice = "Alice".to_string();
    let person = Person::new(&name);

    assert_eq!(person.is_valid_choice(&choice), true);
  }

  #[test]
  fn invalid_choice_without_spouse_is_correct() {
    let name = "Bob".to_string();
    let choice = "Bob".to_string();
    let person = Person::new(&name);

    assert_eq!(person.is_valid_choice(&choice), false);
  }

  #[test]
  fn valid_choice_with_spouse_is_correct() {
    let name = "Bob".to_string();
    let spouse = "Alice".to_string();
    let choice = "Frank".to_string();
    let person = Person::with_spouse(&name, &spouse);

    assert_eq!(person.is_valid_choice(&choice), true);
  }

  #[test]
  fn invalid_choice_with_spouse_is_correct() {
    let name = "Bob".to_string();
    let spouse = "Alice".to_string();
    let choice = "Alice".to_string();
    let person = Person::with_spouse(&name, &spouse);

    assert_eq!(person.is_valid_choice(&choice), false);
  }

  #[test]
  fn to_string_works_without_choice() {
    let name = "Bob".to_string();
    let person = Person::new(&name);

    assert_eq!(person.to_string(), format!("{name} is buying a gift for None"));
  }

  #[test]
  fn to_string_works_with_choice() {
    let name = "Bob".to_string();
    let choice = "Alice".to_string();
    let mut person = Person::new(&name);
    person.choice = Some(choice.clone());

    assert_eq!(person.to_string(), format!("{name} is buying a gift for {choice}"));
  }
}
