#![feature(const_fn_trait_bound)]

mod fe_prelude;
mod fe_std;

use fe_prelude::*;
use fe_std::Console;

#[allow(non_upper_case_globals)]
const STR_SLICE_0: FeString = FeString::from_slice("1");

#[allow(non_upper_case_globals)]
const STR_SLICE_1: FeString = FeString::from_slice("2");

#[allow(non_upper_case_globals)]
const STR_SLICE_2: FeString = FeString::from_slice("3");

#[allow(non_upper_case_globals)]
const STR_SLICE_3: FeString = FeString::from_slice("4");

#[derive(Debug)]
struct Device {
    instance_id: FeShareable<fe_std::UUID>,
    serial: FeShareable<FeString>,
    is_active: bool,
    is_legacy: bool,
}
impl Device {
    pub fn new(
        name: FeShareable<FeString>,
        is_active: Option<bool>,
        is_legacy: Option<bool>,
    ) -> Self {
        let shared = name.share();
        let name = shared.0;

        return Self {
            instance_id: FeShareable::new(fe_std::UUID::from_seed(&shared.1)),
            serial: name,
            is_active: is_active.unwrap_or(true),
            is_legacy: is_legacy.unwrap_or(false),
        };
    }

    pub fn get_name(&self) -> &FeString {
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

    fn on_clone(source: &Device, cloned: &mut Device) {
        cloned.instance_id = FeShareable::new(fe_std::UUID::from_seed(&FeString::from_owned(
            cloned.instance_id.to_string().clone(),
        )));

        Console::write_line(FeString::from_owned(format!(
            "Cloned device {} into device {}",
            source.instance_id, cloned.instance_id,
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

        Self::on_clone(self, &mut cloned);

        return cloned;
    }
}
impl ShareSub for Device {
    fn on_share(&self) {
        Console::write_line(FeString::from_owned(format!(
            "Shared device {}",
            self.instance_id
        )));
    }
}

fn main() {
    let inactive_legacy_device =
        Device::new(FeShareable::new(STR_SLICE_0), Some(false), Some(true));

    let legacy_device = Device::new(FeShareable::new(STR_SLICE_1), None, Some(true));

    let inactive_device = Device::new(FeShareable::new(STR_SLICE_2), Some(false), None);

    let device = Device::new(FeShareable::new(STR_SLICE_3), None, None);

    let x = FeShareable::new(device.clone());

    let shared = x.share();
    let x = shared.0;
    let shared1 = shared.1;

    let shared = x.share();
    let x = shared.0;
    let shared2 = shared.1;
}
