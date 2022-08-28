#![feature(const_fn_trait_bound)]

mod fe_prelude;
mod fe_std;

use fe_prelude::*;
use fe_std::{Console, Map};

#[allow(non_upper_case_globals)]
const STR_SLICE_0: FeString = FeString::from_slice("123");

#[allow(non_upper_case_globals)]
const STR_SLICE_1: FeString = FeString::from_slice("Adam");

#[allow(non_upper_case_globals)]
const STR_SLICE_2: FeString = FeString::from_slice("Bates");

#[allow(non_upper_case_globals)]
const STR_SLICE_3: FeString = FeString::from_slice("124");

#[allow(non_upper_case_globals)]
const STR_SLICE_4: FeString = FeString::from_slice("Madison");

#[allow(non_upper_case_globals)]
const STR_SLICE_5: FeString = FeString::from_slice("Colletti");

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct PersonId {
    pub value: FeShareable<FeString>,
}

#[derive(Debug, Clone)]
struct Person {
    pub id: FeShareable<PersonId>,
    first_name: FeShareable<FeString>,
    last_name: FeShareable<FeString>,
    age: usize,
}

impl Person {
    pub fn new(
        id: FeShareable<PersonId>,
        first_name: FeShareable<FeString>,
        last_name: FeShareable<FeString>,
        age: usize,
    ) -> Self {
        Self {
            id,
            first_name,
            last_name,
            age,
        }
    }
}

#[derive(Debug)]
struct PersonRepositoryError {}

trait PersonRepository {
    fn create_person(
        &mut self,
        person: FeShareable<Person>,
    ) -> Result<fe_std::Void, PersonRepositoryError>;

    fn find_all_people(&self) -> Result<Vec<FeShareable<Person>>, PersonRepositoryError>;

    fn find_person_by_id(
        &self,
        id: FeShareable<PersonId>,
    ) -> Result<Option<FeShareable<Person>>, PersonRepositoryError>;

    fn update_person_by_id(
        &mut self,
        id: FeShareable<PersonId>,
        person: FeShareable<Person>,
    ) -> Result<fe_std::Void, PersonRepositoryError>;

    fn delete_person_by_id(
        &mut self,
        id: FeShareable<PersonId>,
    ) -> Result<fe_std::Void, PersonRepositoryError>;
}

struct LocalPersonRepository {
    map: FeMutField<Map<FeShareable<PersonId>, FeShareable<Person>>>,
}

impl LocalPersonRepository {
    pub fn new(map: Option<Map<FeShareable<PersonId>, FeShareable<Person>>>) -> Self {
        Self {
            map: FeMutField::new(map.unwrap_or(Map::new())),
        }
    }
}

impl PersonRepository for LocalPersonRepository {
    fn create_person(
        &mut self,
        person: FeShareable<Person>,
    ) -> Result<fe_std::Void, PersonRepositoryError> {
        self.map.insert(FeShareable::clone(&person.id), person);
        return Ok(());
    }

    fn find_all_people(&self) -> Result<Vec<FeShareable<Person>>, PersonRepositoryError> {
        return Ok(self.map.values().cloned().collect());
    }

    fn find_person_by_id(
        &self,
        id: FeShareable<PersonId>,
    ) -> Result<Option<FeShareable<Person>>, PersonRepositoryError> {
        return Ok(self.map.get(&id).cloned());
    }

    fn update_person_by_id(
        &mut self,
        id: FeShareable<PersonId>,
        person: FeShareable<Person>,
    ) -> Result<fe_std::Void, PersonRepositoryError> {
        ({
            if self.map.get(&id).is_none() {
                return Err(PersonRepositoryError {});
            }

            self.map.insert(id, person)
        })
        .ok_or(PersonRepositoryError {})?;

        return Ok(());
    }

    fn delete_person_by_id(
        &mut self,
        id: FeShareable<PersonId>,
    ) -> Result<fe_std::Void, PersonRepositoryError> {
        return self
            .map
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
    fn add_person(
        &mut self,
        person: FeShareable<Person>,
    ) -> Result<FeShareable<Person>, PersonServiceError>;
    fn find_or_add_person(
        &mut self,
        person: FeShareable<Person>,
    ) -> Result<FindOrAdd<FeShareable<Person>>, PersonServiceError>;
    fn find_all_people(&self) -> Result<Vec<FeShareable<Person>>, PersonServiceError>;
    fn find_person_by_id(
        &self,
        id: FeShareable<PersonId>,
    ) -> Result<Option<FeShareable<Person>>, PersonServiceError>;
    fn update_person_by_id(
        &mut self,
        id: FeShareable<PersonId>,
        person: FeShareable<Person>,
    ) -> Result<FeShareable<Person>, PersonServiceError>;
    fn delete_person_by_id(
        &mut self,
        id: FeShareable<PersonId>,
    ) -> Result<fe_std::Void, PersonServiceError>;
}

struct StoredPersonService {
    person_repository: Box<dyn PersonRepository>,
}

impl StoredPersonService {
    pub fn new(person_repository: Box<dyn PersonRepository>) -> Self {
        Self { person_repository }
    }

    fn _get_person_by_id(
        &self,
        id: FeShareable<PersonId>,
    ) -> Result<FeShareable<Person>, PersonServiceError> {
        return self
            .person_repository
            .find_person_by_id(id)
            .map_err(|_e| PersonServiceError {})?
            .ok_or_else(|| PersonServiceError {});
    }
}

impl PersonService for StoredPersonService {
    fn add_person(
        &mut self,
        person: FeShareable<Person>,
    ) -> Result<FeShareable<Person>, PersonServiceError> {
        let id = FeShareable::clone(&person.id);

        self.person_repository
            .create_person(person)
            .map_err(|_e| PersonServiceError {})?;

        return self._get_person_by_id(id);
    }

    fn find_or_add_person(
        &mut self,
        person: FeShareable<Person>,
    ) -> Result<FindOrAdd<FeShareable<Person>>, PersonServiceError> {
        let existing = self
            .find_person_by_id(FeShareable::clone(&person.id))
            .map_err(|_e| PersonServiceError {})?;

        if let Some(person) = existing {
            return Ok(FindOrAdd::Found(person));
        }

        return self
            .add_person(person)
            .map(|person| FindOrAdd::Added(person))
            .map_err(|_e| PersonServiceError {});
    }

    fn find_all_people(&self) -> Result<Vec<FeShareable<Person>>, PersonServiceError> {
        return self
            .person_repository
            .find_all_people()
            .map_err(|_e| PersonServiceError {});
    }

    fn find_person_by_id(
        &self,
        id: FeShareable<PersonId>,
    ) -> Result<Option<FeShareable<Person>>, PersonServiceError> {
        return self
            .person_repository
            .find_person_by_id(id)
            .map_err(|_e| PersonServiceError {});
    }

    fn update_person_by_id(
        &mut self,
        id: FeShareable<PersonId>,
        person: FeShareable<Person>,
    ) -> Result<FeShareable<Person>, PersonServiceError> {
        let shared = id.share();
        let id = shared.0;

        self.person_repository
            .update_person_by_id(shared.1, person)
            .map_err(|_e| PersonServiceError {})?;

        return self._get_person_by_id(id);
    }

    fn delete_person_by_id(
        &mut self,
        id: FeShareable<PersonId>,
    ) -> Result<fe_std::Void, PersonServiceError> {
        return self
            .person_repository
            .delete_person_by_id(id)
            .map_err(|_e| PersonServiceError {});
    }
}

fn main() -> Result<fe_std::Void, PersonServiceError> {
    let mut person_repository = LocalPersonRepository::new(None);

    let mut person_service: Box<dyn PersonService> =
        Box::new(StoredPersonService::new(Box::new(person_repository)));

    let mut person1 = Person::new(
        FeShareable::new(PersonId {
            value: FeShareable::new(STR_SLICE_0),
        }),
        FeShareable::new(STR_SLICE_1),
        FeShareable::new(STR_SLICE_2),
        24,
    );

    let mut person2 = FeShareable::new(Person::new(
        FeShareable::new(PersonId {
            value: FeShareable::new(STR_SLICE_3),
        }),
        FeShareable::new(STR_SLICE_4),
        FeShareable::new(STR_SLICE_5),
        23,
    ));

    let added1 = person_service.add_person(FeShareable::new(person1.clone()))?;
    Console::write_line(FeString::from_owned(format!(
        "Added person id {}",
        added1.id.value
    )));

    let shared = person2.share();
    let person2 = shared.0;

    let found_or_added1 = person_service.find_or_add_person(shared.1)?;
    match found_or_added1 {
        FindOrAdd::Found(found) => Console::write_line(FeString::from_owned(format!(
            "Found person id {}",
            found.id.value
        ))),
        FindOrAdd::Added(added) => Console::write_line(FeString::from_owned(format!(
            "Added person id {}",
            added.id.value
        ))),
    };

    let shared = person2.share();
    let person2 = shared.0;

    let found_or_added2 = person_service.find_or_add_person(shared.1)?;
    match found_or_added2 {
        FindOrAdd::Found(found) => Console::write_line(FeString::from_owned(format!(
            "Found person id {}",
            found.id.value
        ))),
        FindOrAdd::Added(added) => Console::write_line(FeString::from_owned(format!(
            "Added person id {}",
            added.id.value
        ))),
    };

    person1.age = 25;

    let shared = FeShareable::new(person1).share();
    let person1 = shared.0;

    let updated1 = person_service.update_person_by_id(FeShareable::clone(&person1.id), shared.1)?;
    Console::write_line(FeString::from_owned(format!(
        "Updated person id {}",
        updated1.id.value
    )));

    person_service.delete_person_by_id(FeShareable::clone(&person2.id))?;
    Console::write_line(FeString::from_owned(format!(
        "Deleted person id {}",
        person2.id.value
    )));

    return Ok(());
}
