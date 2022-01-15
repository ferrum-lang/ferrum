mod lang_prelude;
mod lang_std;

use lang_prelude::*;
use lang_std::Console;

#[allow(non_upper_case_globals)]
const STR_SLICE_0: LangString = LangString::from_slice("1");

#[allow(non_upper_case_globals)]
const STR_SLICE_1: LangString = LangString::from_slice("2");

#[allow(non_upper_case_globals)]
const STR_SLICE_2: LangString = LangString::from_slice("3");

#[allow(non_upper_case_globals)]
const STR_SLICE_3: LangString = LangString::from_slice("4");

struct Device {
  instance_id: lang_std::UUID,
  serial: LangString,
  is_active: bool,
  is_legacy: bool,
}
impl Device {
  pub fn new(name: LangString, is_active: Option<bool>, is_legacy: Option<bool>) -> Self {
    let mut s = Self {
      instance_id: lang_std::UUID::from_seed(&name),
      serial: name,
      is_active: is_active.unwrap_or(true),
      is_legacy: is_legacy.unwrap_or(false),
    };

    Self::on_create(&mut s);

    return s;
  }

  pub fn get_name(&self) -> &LangString {
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

  fn on_create(created: &mut Device) {
    Console::write_line(LangString::from_owned(format!(
      "Created device {}",
      created.instance_id
    )));
  }

  fn on_drop(dropped: &mut Device) {
    Console::write_line(LangString::from_owned(format!(
      "Dropping device {}",
      dropped.instance_id
    )));
  }

  fn on_clone(source: &Device, cloned: &mut Device) {
    cloned.instance_id = lang_std::UUID::from_seed(&LangString::from_owned(
      cloned.instance_id.to_string().clone(),
    ));

    Console::write_line(LangString::from_owned(format!(
      "Cloned device {} into device {}",
      source.instance_id, cloned.instance_id,
    )));
  }

  fn on_share(shared: &Device) {
    Console::write_line(LangString::from_owned(format!(
      "Shared device {}",
      shared.instance_id
    )));
  }
}
impl std::ops::Drop for Device {
  fn drop(&mut self) {
    Self::on_drop(self);
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

    Self::on_clone(self, &mut cloned);

    return cloned;
  }
}
impl Share<Device> for Device {
  fn on_share(&self) {
    Self::on_share(self);
  }
}

fn main() {
  let inactive_legacy_device = Device::new(STR_SLICE_0, Some(false), Some(true));

  let legacy_device = Device::new(STR_SLICE_1, None, Some(true));

  let inactive_device = Device::new(STR_SLICE_2, Some(false), None);

  let device = Device::new(STR_SLICE_3, None, None);

  let x = Shareable::new(device.clone());

  let share1 = x.share();
  let borrow1 = share1.borrow();

  let share2 = x.share();
  let borrow2 = share2.borrow();
}
