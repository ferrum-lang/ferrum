mod args;
mod compiler;
mod config;
mod executor;
mod io;
mod utils;

use anyhow::Result;

fn main() -> Result<()> {
    temp_main()?;

    let args = args::parse_args();
    
    let config = config::build(args);

    let output_contents = compiler::compile(&config.input_filepath)?;

    io::write_to_build_dir(&config, output_contents)?;

    return executor::build_and_run(&config);
}

fn temp_main() -> Result<()> {
    let name = "Adam";

    println!("Hello, {name}");

    return Ok(());
}

/*

fn temp_main() -> ! {
    const name = "Adam"

    print("Hello, {name}")
}

*/

struct TodoItem {
    pub title: String,
    pub description: Option<String>,
    pub is_complete: bool,
}

struct TodoList {
    list: Vec<TodoItem>,
}

impl TodoList {
    pub fn new() -> Self {
        return Self {
            list: Vec::new(),
        };
    }

    pub fn add_item(&mut self, title: String, description: Option<String>) -> usize {
        self.list.push(TodoItem { title, description, is_complete: false });
        return self.list.len() - 1;
    }

    pub fn complete_item(&mut self, id: usize) {
        self.list[id].is_complete = true;
    }

    pub fn remove_item(&mut self, id: usize) -> TodoItem {
        return self.list.remove(id);
    }

    pub fn borrow_item(&self, id: usize) -> Option<&TodoItem> {
        return self.list.get(id);
    }

    pub fn borrow_mut_item(&mut self, id: usize) -> Option<&mut TodoItem> {
        return self.list.get_mut(id);
    }

    pub fn take_item(&mut self, id: usize) -> Option<TodoItem> {
        if id < self.list.len() {
            return None;
        }

        return Some(self.list.remove(id));
    }

    pub fn find_id(&self, test: impl Fn(&TodoItem) -> bool) -> Option<usize> {
        for (id, item) in self.list.iter().enumerate() {
            if test(item) {
                return Some(id);
            }
        }

        return None;
    }
}

fn temp_todos() {
    let mut todos = TodoList::new();

    let id = todos.add_item("Finish lang".to_string(), None);

    todos.complete_item(id);

    {
        let x: Option<&TodoItem> = todos.borrow_item(id);
    }

    {
        let x: Option<&mut TodoItem> = todos.borrow_mut_item(id);
    }

    todos.remove_item(id);

    {
        let x: Option<TodoItem> = todos.take_item(id);
    }

    let x: Option<usize> = todos.find_id(|item| item.title.len() > 0);
}

/*

struct TodoItem {
    title: string,
    description: string?,
    is_complete: bool = false,
}

class TodoList {
    self {
        const list: mut+ [TodoItem] = [],
    }

    pub mut self.add_item(title: string, description: string?) -> uint {
        return self.list.push(TodoItem(title, description))
    }

    pub mut self.complete_item(id: uint) -> ! {
        self.list[id]!.is_complete = true
    }

    pub mut self.remove_item(id: uint) -> ! {
        self.list.remove(id)!
    }

    pub self.borrow_item(id: uint) -> TodoItem? {
        return self.list.get(id)
    }

    pub mut self.borrow_mut_item(id: uint) -> mut TodoItem? {
        return self.list.get_mut(id)
    }

    pub mut self.take_item(id: uint) -> mut TodoItem? {
        return self.list.remove(id)
    }

    pub self.find_id(test: fn(&TodoItem) -> bool) -> uint? {
        for item, id in self.list {
            if test(item) {
                return id
            }
        }
    }
}

fn temp_todos() -> ! {
    const todos = mut TodoList()

    const id = todos.add_item("Finish lang")

    todos.complete_item(id)!

    {
        const x: TodoItem? = todos.borrow_item(id)
    }

    {
        const x: mut TodoItem? = mut todos.borrow_mut_item(id)
    }

    todos.remove_item(id)!

    {
        const x: TodoItem? = todos.take_item(id)
    }

    const x: uint? = todos.find_id((item) => item.title.len() > 0)
}

*/

