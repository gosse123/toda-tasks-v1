# Rust Todo CLI

A simple and modular command-line task manager written in Rust.

This project allows users to create, view, update, and delete tasks directly from the terminal. It was built to practice Rust fundamentals such as modules, structs, implementations, vectors, error handling, ownership, and user input management.

## Features

* Add new tasks
* Display all tasks
* Mark tasks as completed
* Delete tasks
* Interactive command-line interface
* Modular project architecture
* Error handling for user input

## Project Structure

```text
src/
├── main.rs
├── todo/
│   └── todo_struct.rs
└── utils/
    └── fn_lecture.rs
```

### Modules

#### `main.rs`

Application entry point.

Responsible for:

* Displaying the menu
* Handling user choices
* Managing the application loop

#### `todo`

Contains:

* Task structure definition
* Task-related implementations
* CRUD operations

#### `utils`

Contains helper functions such as:

* Reading user input
* Input validation
* Integer parsing

## Task Model

Each task contains:

```rust
id: u32
name: String
status: bool
```

Where:

* `id` uniquely identifies a task
* `name` stores the task description
* `status` indicates whether the task is completed

## Available Commands

```text
1 Add a task
2 Show tasks
3 Mark task as completed
4 Delete task
5 Exit
```

## Getting Started

### Requirements

* Rust
* Cargo

### Build

```bash
cargo build
```

### Run

```bash
cargo run
```

## Example Usage

```text
1 Add a task
2 Show tasks
3 Mark task as completed
4 Delete task
5 Exit

> 1

Enter task name:
Learn Rust

Task added successfully.
```

## Technical Concepts Demonstrated

This project demonstrates:

* Rust modules
* Structs
* Implementations (`impl`)
* Vectors (`Vec<T>`)
* Ownership and borrowing
* Mutable references
* Error handling with `Result`
* Pattern matching with `match`
* CLI application design

## Future Improvements

### Persistence

* [ ] Save tasks to a file
* [ ] Load tasks on startup
* [ ] JSON serialization with serde

### User Experience

* [ ] Colored terminal output
* [ ] Better menu interface
* [ ] Search functionality
* [ ] Task filtering

### Architecture

* [ ] Separate business logic from presentation layer
* [ ] Add unit tests
* [ ] Add integration tests

### Advanced Features

* [ ] Task priorities
* [ ] Due dates
* [ ] Categories
* [ ] Tags
* [ ] Export to CSV

## Learning Objectives

This project was created to learn:

* Rust fundamentals
* Modular application design
* Data management using vectors
* Error handling patterns
* Command-line application development

## License

MIT License

## Author

Nahounou Gosse

Computer Science Student passionate about Rust, Systems Programming, Artificial Intelligence, and Cybersecurity.
