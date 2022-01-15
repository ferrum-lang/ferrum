mod lang_prelude;
mod lang_std;

use lang_prelude::*;
use lang_std::{Console, Map};

#[allow(non_upper_case_globals)]
const STR_SLICE_0: LangString = LangString::from_slice("123");

#[allow(non_upper_case_globals)]
const STR_SLICE_1: LangString = LangString::from_slice("Adam");

#[allow(non_upper_case_globals)]
const STR_SLICE_2: LangString = LangString::from_slice("Bates");

#[allow(non_upper_case_globals)]
const STR_SLICE_3: LangString = LangString::from_slice("124");

#[allow(non_upper_case_globals)]
const STR_SLICE_4: LangString = LangString::from_slice("Madison");

#[allow(non_upper_case_globals)]
const STR_SLICE_5: LangString = LangString::from_slice("Colletti");

#[allow(non_upper_case_globals)]
const STR_SLICE_6: LangString = LangString::from_slice("1");

#[allow(non_upper_case_globals)]
const STR_SLICE_7: LangString = LangString::from_slice("2");

#[allow(non_upper_case_globals)]
const STR_SLICE_8: LangString = LangString::from_slice("3");

#[allow(non_upper_case_globals)]
const STR_SLICE_9: LangString = LangString::from_slice("4");

#[derive(PartialEq, Eq, Hash)]
struct PersonId {
    pub value: LangString,
}
impl Share<PersonId> for PersonId {}

struct Person {
    pub id: Shareable<PersonId>,
    first_name: LangString,
    last_name: LangString,
    age: usize,
}
impl Share<Person> for Person {}

impl Person {
    pub fn new(
        id: Shareable<PersonId>,
        first_name: LangString,
        last_name: LangString,
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
impl Share<PersonRepositoryError> for PersonRepositoryError {}

trait PersonRepository {
    fn create_person(
        &mut self,
        person: Shareable<Person>,
    ) -> Result<lang_std::Void, PersonRepositoryError>;
    fn find_all_people(&self) -> Result<Vec<Shareable<Person>>, PersonRepositoryError>;
    fn find_person_by_id(
        &self,
        id: Shareable<PersonId>,
    ) -> Result<Option<Shareable<Person>>, PersonRepositoryError>;
    fn update_person_by_id(
        &mut self,
        id: Shareable<PersonId>,
        person: Shareable<Person>,
    ) -> Result<lang_std::Void, PersonRepositoryError>;
    fn delete_person_by_id(
        &mut self,
        id: Shareable<PersonId>,
    ) -> Result<lang_std::Void, PersonRepositoryError>;
}

struct LocalPersonRepository {
    map: Map<Shareable<PersonId>, Shareable<Person>>,
}
impl Share<LocalPersonRepository> for LocalPersonRepository {}

impl LocalPersonRepository {
    pub fn new(map: Option<Map<Shareable<PersonId>, Shareable<Person>>>) -> Self {
        Self {
            map: map.unwrap_or(Map::new()),
        }
    }
}

impl PersonRepository for LocalPersonRepository {
    fn create_person(
        &mut self,
        person: Shareable<Person>,
    ) -> Result<lang_std::Void, PersonRepositoryError> {
        self.map.insert(person.borrow().id.share(), person.share());
        return Ok(());
    }

    fn find_all_people(&self) -> Result<Vec<Shareable<Person>>, PersonRepositoryError> {
        return Ok(self
            .map
            .values()
            .into_iter()
            .map(|v| v.share())
            .collect::<Vec<Shareable<Person>>>());
    }

    fn find_person_by_id(
        &self,
        id: Shareable<PersonId>,
    ) -> Result<Option<Shareable<Person>>, PersonRepositoryError> {
        return Ok(self.map.get(&id).map(|v| v.share()));
    }

    fn update_person_by_id(
        &mut self,
        id: Shareable<PersonId>,
        person: Shareable<Person>,
    ) -> Result<lang_std::Void, PersonRepositoryError> {
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
        id: Shareable<PersonId>,
    ) -> Result<lang_std::Void, PersonRepositoryError> {
        return self
            .map
            .remove(&id)
            .ok_or(PersonRepositoryError {})
            .map(|_| ());
    }
}

#[derive(Debug)]
struct PersonServiceError {}
impl Share<PersonServiceError> for PersonServiceError {}

enum FindOrAdd<T> {
    Found(T),
    Added(T),
}
impl<T> Share<FindOrAdd<T>> for FindOrAdd<T> {}

trait PersonService {
    fn add_person(
        &mut self,
        person: Shareable<Person>,
    ) -> Result<Shareable<Person>, PersonServiceError>;
    fn find_or_add_person(
        &mut self,
        person: Shareable<Person>,
    ) -> Result<FindOrAdd<Shareable<Person>>, PersonServiceError>;
    fn find_all_people(&self) -> Result<Vec<Shareable<Person>>, PersonServiceError>;
    fn find_person_by_id(
        &self,
        id: Shareable<PersonId>,
    ) -> Result<Option<Shareable<Person>>, PersonServiceError>;
    fn update_person_by_id(
        &mut self,
        id: Shareable<PersonId>,
        person: Shareable<Person>,
    ) -> Result<Shareable<Person>, PersonServiceError>;
    fn delete_person_by_id(
        &mut self,
        id: Shareable<PersonId>,
    ) -> Result<lang_std::Void, PersonServiceError>;
}

struct StoredPersonService {
    person_repository: Box<dyn PersonRepository>,
}
impl Share<StoredPersonService> for StoredPersonService {}

impl StoredPersonService {
    pub fn new(person_repository: Box<dyn PersonRepository>) -> Self {
        Self { person_repository }
    }

    fn _get_person_by_id(
        &self,
        id: Shareable<PersonId>,
    ) -> Result<Shareable<Person>, PersonServiceError> {
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
        person: Shareable<Person>,
    ) -> Result<Shareable<Person>, PersonServiceError> {
        let id = person.borrow().id.share();

        self.person_repository
            .create_person(person)
            .map_err(|_e| PersonServiceError {})?;

        return self._get_person_by_id(id);
    }

    fn find_or_add_person(
        &mut self,
        person: Shareable<Person>,
    ) -> Result<FindOrAdd<Shareable<Person>>, PersonServiceError> {
        let existing = self
            .find_person_by_id(person.borrow().id.share())
            .map_err(|_e| PersonServiceError {})?;

        if let Some(person) = existing {
            return Ok(FindOrAdd::Found(person));
        }

        return self
            .add_person(person)
            .map(|person| FindOrAdd::Added(person))
            .map_err(|_e| PersonServiceError {});
    }

    fn find_all_people(&self) -> Result<Vec<Shareable<Person>>, PersonServiceError> {
        return self
            .person_repository
            .find_all_people()
            .map_err(|_e| PersonServiceError {});
    }

    fn find_person_by_id(
        &self,
        id: Shareable<PersonId>,
    ) -> Result<Option<Shareable<Person>>, PersonServiceError> {
        return self
            .person_repository
            .find_person_by_id(id)
            .map_err(|_e| PersonServiceError {});
    }

    fn update_person_by_id(
        &mut self,
        id: Shareable<PersonId>,
        person: Shareable<Person>,
    ) -> Result<Shareable<Person>, PersonServiceError> {
        self.person_repository
            .update_person_by_id(id.share(), person)
            .map_err(|_e| PersonServiceError {})?;

        return self._get_person_by_id(id);
    }

    fn delete_person_by_id(
        &mut self,
        id: Shareable<PersonId>,
    ) -> Result<lang_std::Void, PersonServiceError> {
        return self
            .person_repository
            .delete_person_by_id(id)
            .map_err(|_e| PersonServiceError {});
    }
}

struct Device {
    instance_id: lang_std::UUID,
    serial: LangString,
    is_active: bool,
    is_legacy: bool,
}
impl Device {
    pub fn new(serial: LangString, is_active: Option<bool>, is_legacy: Option<bool>) -> Self {
        let mut s = Self {
            instance_id: lang_std::UUID::from_seed(&serial),
            serial,
            is_active: is_active.unwrap_or(true),
            is_legacy: is_legacy.unwrap_or(false),
        };

        Self::on_create(&mut s);

        return s;
    }

    pub fn get_serial(&self) -> &LangString {
        return &self.serial;
    }

    pub fn get_is_active(&self) -> &bool {
        return &self.is_active;
    }

    pub fn get_is_legacy(&self) -> &bool {
        return &self.is_legacy;
    }

    pub fn set_is_active(&mut self, is_active: bool) {
        self.is_active = is_active;
    }

    pub fn on_create(created: &mut Device) {
        Console::write_line(LangString::from_owned(format!(
            "Created device {}",
            created.instance_id
        )));
    }

    pub fn on_clone(source: &Device, cloned: &mut Device) {
        cloned.instance_id = lang_std::UUID::from_seed(&LangString::from_owned(
            cloned.instance_id.to_string().clone(),
        ));

        Console::write_line(LangString::from_owned(format!(
            "Cloned device {} into device {}",
            source.instance_id, cloned.instance_id,
        )));
    }
}
impl std::ops::Drop for Device {
    fn drop(&mut self) {
        Console::write_line(LangString::from_owned(format!(
            "Dropping device: {}",
            self.instance_id
        )));
    }
}
impl std::clone::Clone for Device {
    fn clone(&self) -> Self {
        let mut cloned = Self {
            instance_id: self.instance_id.clone(),
            serial: self.serial.clone(),
            is_active: self.is_active.clone(),
            is_legacy: self.is_legacy.clone(),
        };

        Device::on_clone(self, &mut cloned);

        return cloned;
    }
}
impl Share<Device> for Device {
    fn on_share(instance: Instance<Device>) {
        match instance {
            Instance::Mutable(mut this) => Console::write_line(LangString::from_owned(format!(
                "Mutably shared device {}",
                this.instance_id
            ))),
            Instance::Immutable(this) => Console::write_line(LangString::from_owned(format!(
                "Immutably shared device {}",
                this.instance_id
            ))),
        }
    }
}

fn main() -> Result<lang_std::Void, PersonServiceError> {
    let mut person_repository = LocalPersonRepository::new(None);

    let mut person_service: Box<dyn PersonService> =
        Box::new(StoredPersonService::new(Box::new(person_repository)));

    let mut person1 = Shareable::new(Person::new(
        Shareable::new(PersonId { value: STR_SLICE_0 }),
        STR_SLICE_1,
        STR_SLICE_2,
        24,
    ));

    let mut person2 = Shareable::new(Person::new(
        Shareable::new(PersonId { value: STR_SLICE_3 }),
        STR_SLICE_4,
        STR_SLICE_5,
        23,
    ));

    let added1 = person_service.add_person(person1.share())?;
    Console::write_line(LangString::from_owned(format!(
        "Added person id {}",
        added1.borrow().id.borrow().value
    )));

    let found_or_added1 = person_service.find_or_add_person(person2.share())?;
    match found_or_added1 {
        FindOrAdd::Found(found) => Console::write_line(LangString::from_owned(format!(
            "Found person id {}",
            found.borrow().id.borrow().value
        ))),
        FindOrAdd::Added(added) => Console::write_line(LangString::from_owned(format!(
            "Added person id {}",
            added.borrow().id.borrow().value
        ))),
    };

    let found_or_added2 = person_service.find_or_add_person(person2.share())?;
    match found_or_added2 {
        FindOrAdd::Found(found) => Console::write_line(LangString::from_owned(format!(
            "Found person id {}",
            found.borrow().id.borrow().value
        ))),
        FindOrAdd::Added(added) => Console::write_line(LangString::from_owned(format!(
            "Added person id {}",
            added.borrow().id.borrow().value
        ))),
    };

    person1.borrow_mut().age = 25;
    let updated1 =
        person_service.update_person_by_id(person1.borrow().id.share(), person1.share())?;
    Console::write_line(LangString::from_owned(format!(
        "Updated person id {}",
        updated1.borrow().id.borrow().value
    )));

    person_service.delete_person_by_id(person2.borrow().id.share())?;
    Console::write_line(LangString::from_owned(format!(
        "Deleted person id {}",
        person2.borrow().id.borrow().value
    )));

    let inactive_legacy_device = Device::new(STR_SLICE_6, Some(false), Some(true));

    let legacy_device = Device::new(STR_SLICE_7, None, Some(true));

    let inactive_device = Device::new(STR_SLICE_8, Some(false), None);

    let device = Device::new(STR_SLICE_9, None, None);

    let x = Shareable::new(device.clone());

    {
        let share1 = x.share(); // calls on_share(Mutable)
        let borrow1 = share1.borrow();
        let share2 = x.share();
        let borrow2 = share2.borrow();
    }
    {
        let share1 = x.share();
        let share2 = x.share();
    }

    return Ok(());
}
