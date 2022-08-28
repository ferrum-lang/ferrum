mod ferrum_rs;
use ferrum_rs::prelude::*;

fn main() {
    let name: FeStr = FeStr::from("Adam");
    print(&name);

    let name1: FeBox<FeStr> = FeBox::new(name);
    print(&name1);

    let mut name2: FeBox<FeStr> = FeBox::share(&name1);

    let mut name3: FeBox<FeStr> = FeBox::share(&name1);

    print(&name1);

    *name2 = FeStr::from("Foo");
    print(&name1);

    *name3 = FeStr::from("Bar");
    print(name1);
}

