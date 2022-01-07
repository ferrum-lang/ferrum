mod lang_prelude;
mod lang_std;

use lang_prelude::*;
use lang_std::{ Console, Map };

#[derive(Clone, PartialEq, Eq, Hash)]
struct PersonId {
  pub value: LangString,
}

#[derive(Clone)]
struct Person {
  id: PersonId,
  first_name: LangString,
  last_name: LangString,
  age: usize,
}

impl Person {
  pub fn new(id: PersonId, first_name: LangString, last_name: LangString, age: usize) -> Self {
    Self { id, first_name, last_name, age }
  }
}

type People = Vec<Person>;

#[derive(Debug)]
struct PersonRepositoryError {}

trait PersonRepository {
  fn create_person(&mut self, person: Person) -> Result<lang_std::Void, PersonRepositoryError>;
  fn find_all_people(&self) -> Result<People, PersonRepositoryError>;
  fn find_person_by_id(&self, id: PersonId) -> Result<Option<Person>, PersonRepositoryError>;
  fn update_person_by_id(&mut self, id: PersonId, person: Person) -> Result<lang_std::Void, PersonRepositoryError>;
  fn delete_person_by_id(&mut self, id: PersonId) -> Result<lang_std::Void, PersonRepositoryError>;
}

struct LocalPersonRepository {
  map: Map<PersonId, Person>,
}

impl LocalPersonRepository {
  pub fn new(map: Option<Map<PersonId, Person>>) -> Self {
    Self {
      map: map.unwrap_or(Map::new()),
    }
  }
}

impl PersonRepository for LocalPersonRepository {
  fn create_person(&mut self, person: Person) -> Result<lang_std::Void, PersonRepositoryError> {
    self.map.insert(person.id.clone(), person);
    return Ok(());
  }

  fn find_all_people(&self) -> Result<People, PersonRepositoryError> {
    return Ok(
      self.map.values().cloned().collect::<People>()
    );
  }

  fn find_person_by_id(&self, id: PersonId) -> Result<Option<Person>, PersonRepositoryError> {
    return Ok(self.map.get(&id).cloned());
  }

  fn update_person_by_id(&mut self, id: PersonId, person: Person) -> Result<lang_std::Void, PersonRepositoryError> {
    ({
      if self.map.get(&id).is_none() {
        return Err(PersonRepositoryError {});
      }

      self.map
        .insert(id, person)
    })
      .ok_or(PersonRepositoryError {})?;

    return Ok(());
  }

  fn delete_person_by_id(&mut self, id: PersonId) -> Result<lang_std::Void, PersonRepositoryError> {
    return self.map
      .remove(&id)
      .ok_or(PersonRepositoryError {})
      .map(|_| ());
  }
}

#[derive(Debug)]
struct PersonServiceError {}

enum FindOrAdd<T> {
  Found(T),
  Added(T),
}

trait PersonService {
  fn add_person(&mut self, person: Person) -> Result<Person, PersonServiceError>;
  fn find_or_add_person(&mut self, person: Person) -> Result<FindOrAdd<Person>, PersonServiceError>;
  fn find_all_people(&self) -> Result<People, PersonServiceError>;
  fn find_person_by_id(&self, id: PersonId) -> Result<Option<Person>, PersonServiceError>;
  fn update_person_by_id(&mut self, id: PersonId, person: Person) -> Result<Person, PersonServiceError>;
  fn delete_person_by_id(&mut self, id: PersonId) -> Result<lang_std::Void, PersonServiceError>;
}

struct StoredPersonService {
  person_repository: Box<dyn PersonRepository>,
}

impl StoredPersonService {
  pub fn new(person_repository: Box<dyn PersonRepository>) -> Self {
    Self { person_repository }
  }

  fn _get_person_by_id(&self, id: PersonId) -> Result<Person, PersonServiceError> {
    return self.person_repository
      .find_person_by_id(id)
      .map_err(|_e| PersonServiceError {})?
      .ok_or_else(|| PersonServiceError {});
  }
}

impl PersonService for StoredPersonService {
  fn add_person(&mut self, person: Person) -> Result<Person, PersonServiceError> {
    let id = person.id.clone();

    self.person_repository
      .create_person(person)
      .map_err(|_e| PersonServiceError {})?;
    
    return self._get_person_by_id(id);
  }

  fn find_or_add_person(&mut self, person: Person) -> Result<FindOrAdd<Person>, PersonServiceError> {
    let existing = self
      .find_person_by_id(person.id.clone())
      .map_err(|_e| PersonServiceError {})?;

    if let Some(person) = existing {
      return Ok(FindOrAdd::Found(person));
    }

    return self.add_person(person)
      .map(|person| FindOrAdd::Added(person))
      .map_err(|_e| PersonServiceError {});
  }

  fn find_all_people(&self) -> Result<People, PersonServiceError> {
    return self.person_repository
      .find_all_people()
      .map_err(|_e| PersonServiceError {});
  }

  fn find_person_by_id(&self, id: PersonId) -> Result<Option<Person>, PersonServiceError> {
    return self.person_repository
      .find_person_by_id(id)
      .map_err(|_e| PersonServiceError {});
  }

  fn update_person_by_id(&mut self, id: PersonId, person: Person) -> Result<Person, PersonServiceError> {
    self.person_repository
      .update_person_by_id(id.clone(), person)
      .map_err(|_e| PersonServiceError {})?;
    
    return self._get_person_by_id(id);
  }

  fn delete_person_by_id(&mut self, id: PersonId) -> Result<lang_std::Void, PersonServiceError> {
    return self.person_repository
      .delete_person_by_id(id)
      .map_err(|_e| PersonServiceError {});
  }
}

fn main() -> Result<lang_std::Void, PersonServiceError> {
  let mut person_repository = LocalPersonRepository::new(None);
  
  let mut person_service = StoredPersonService::new(Box::new(person_repository));

  let mut person1 = Person::new(
    PersonId { value: LangString::from_slice("123") },
    LangString::from_slice("Adam"),
    LangString::from_slice("Bates"),
    24,
  );

  let mut person2 = Person::new(
    PersonId { value: LangString::from_slice("124") },
    LangString::from_slice("Madison"),
    LangString::from_slice("Colletti"),
    23,
  );

  let added1 = person_service.add_person(person1.clone())?;
  Console::write_line(LangString::from_owned(format!("Added person id {}", added1.id.value)));

  let found_or_added1 = person_service.find_or_add_person(person2.clone())?;
  match found_or_added1 {
    FindOrAdd::Found(found) => Console::write_line(LangString::from_owned(format!("Found person id {}", found.id.value))),
    FindOrAdd::Added(added) => Console::write_line(LangString::from_owned(format!("Added person id {}", added.id.value))),
  };

  let found_or_added2 = person_service.find_or_add_person(person2.clone())?;
  match found_or_added2 {
    FindOrAdd::Found(found) => Console::write_line(LangString::from_owned(format!("Found person id {}", found.id.value))),
    FindOrAdd::Added(added) => Console::write_line(LangString::from_owned(format!("Added person id {}", added.id.value))),
  };

  person1.age = 25;
  let updated1 = person_service.update_person_by_id(person1.id.clone(), person1.clone())?;
  Console::write_line(LangString::from_owned(format!("Updated person id {}", updated1.id.value)));

  person_service.delete_person_by_id(person2.id.clone())?;
  Console::write_line(LangString::from_owned(format!("Deleted person id {}", person2.id.value)));

  return Ok(());
}
