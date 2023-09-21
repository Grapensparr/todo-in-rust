use std::env;
use std::fs;
use std::path::Path;

fn main() {
  let mut to_do_list = ToDoList::new();

  let args: Vec<String> = env::args().collect();

  if args.len() > 1 {
    let command = &args[1];

    match command.as_str() {
      "add" => {
        if args.len() > 2 {
          let task_description = args[2..].join(" ");

          if !to_do_list.task_exists(&task_description) {
            to_do_list.add_task(&task_description);
            to_do_list.save_tasks();

            println!("Task added: {}", task_description);
          } else {
            println!("This task already exists: {}", task_description);
          }
        } else {
          println!("Please provide a task description.");
        }
      }

      "list" => {
        to_do_list.list_tasks();
      }

      "remove" => {
        if args.len() > 2 {
          let task_index: usize = match args[2].parse() {
            Ok(index) => index,
            Err(_) => {
              println!("Invalid task index.");
              return;
            }
          };
          if to_do_list.remove_task(task_index) {
            to_do_list.save_tasks();
            println!("Task removed.");
          } else {
            println!("Task not found.");
          }
        } else {
          println!("Please provide a task index to remove.");
        }
      }
      
      _ => {
        println!("Invalid command. Use 'add', 'list', or 'remove'.");
      }
    }
  } else {
    println!("Please use the following commands to run the application:");
    println!("Add a new task: cargo run -- add <task_description>");
    println!("List all tasks: cargo run -- list");
    println!("Remove a task by index: cargo run -- remove <task_index>");
  }
}

struct ToDoList {
  tasks: Vec<String>,
}

impl ToDoList {
  fn new() -> ToDoList {
    let tasks = ToDoList::load_tasks();

    ToDoList { tasks }
  }

  fn load_tasks() -> Vec<String> {
    let path = ToDoList::get_data_file_path();

    match fs::read_to_string(&path) {
      Ok(contents) => contents.lines().map(|s| s.to_string()).collect(),
      Err(_) => vec![],
    }
  }

  fn save_tasks(&self) {
    let path = ToDoList::get_data_file_path();

    if let Err(err) = fs::write(&path, self.tasks.join("\n")) {
      eprintln!("Error saving tasks: {}", err);
    }
  }

  fn task_exists(&self, task: &str) -> bool {
    self.tasks.iter().any(|existing_task| existing_task == task)
  }

  fn add_task(&mut self, task: &str) {
    self.tasks.push(task.to_string());
  }

  fn list_tasks(&self) {
    if self.tasks.is_empty() {
      println!("No tasks.");
    } else {
      for (index, task) in self.tasks.iter().enumerate() {
        println!("{}: {}", index + 1, task);
      }
    }
  }

  fn remove_task(&mut self, index: usize) -> bool {
    if index >= 1 && index <= self.tasks.len() {
      self.tasks.remove(index - 1);
      self.save_tasks();
      true
    } else {
      false
    }
  }

  fn get_data_file_path() -> String {
    let home_dir = match env::var("USERPROFILE").or_else(|_| env::var("HOME")) {
      Ok(path) => path,
      Err(_) => {
        eprintln!("Unable to determine the home directory.");
        String::from(".")
      }
    };

    let data_dir = format!("{}/.todo", home_dir);

    let mut data_file_path = Path::new(&data_dir).to_path_buf();
    data_file_path.push("tasks.txt");

    if !Path::new(&data_dir).exists() {
      if let Err(err) = fs::create_dir(&data_dir) {
        eprintln!("Error creating data directory: {}", err);
      }
    }

    data_file_path.to_string_lossy().to_string()
  }
}