# Company HR Directory (Rust CLI)

This is a simple command-line Rust project that simulates a company's
Human Resource system. It allows you to:

- Add employees to departments.
- Retrieve a list of employees in a specific department.
- Retrieve all employees in the company, grouped and sorted by department.

This project demonstrates my journey learning the use of:
- `HashMap` for associating departments with employee lists.
- `Vec` for storing employees in each department.
- `Enum`, `Structs` and `Modules` that I used for Custom types and
manage structure of project (`Employee`, `Department`) in Rust.
- User input via the terminal.

---

## 🛠 Features

- Add employees to predefined departments (e.g., Engineering, Sales).
- View all employees in a single department.
- View all employees across all departments alphabetically sorted.

---

## 🧱 Project Structure
```text
src/
|-- main.rs         # Program entry point, user interaction loop
|-- employee.rs     # Employee struct and related logic
|
|-- department.rs   # Department enum
```

## 🚀  Getting Started

1. Clone the repository:

```bash
git clone https://github.com/yourusername/hrust.git
cd hrust
```

2. Build and run the project:

```bash
cargo build
cargo run
```

## 🧪 Example Usage

```bash
Welcome to the Human Resource Department
What would you like to do?
1. Add Employee
2. Retrieve employees by department
3. Retrieve all employees
Enter choice:
```

## 🧠 Concepts Practiced

- Hashing with `HashMap`
- Structs and Enums
- Pattern matching
- User input and output
- Modular Rust project layout

## 📁 Departments Included

- Engineering
- Sales
- Marketing
- Finance
- Operations

## ✍️ Author

- `leocm889`
