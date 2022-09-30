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
