use crate::department::Department;
use std::{
    collections::HashMap,
    io::{self, Write},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Employee {
    name: String,
    department: Department,
}

impl Employee {
    pub fn new(name: String, department: Department) -> Self {
        Self { name, department }
    }
}

pub fn add_employee(company: &mut HashMap<Department, Vec<Employee>>) {
    loop {
        println!("Choose department to add employee");
        println!("1. Engineering");
        println!("2. Sales");
        println!("3. Marketing");
        println!("4. Finance");
        println!("5. Operations");

        println!("Enter choice: ");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        let department = match choice {
            1 => Department::Engineering,
            2 => Department::Sales,
            3 => Department::Marketing,
            4 => Department::Finance,
            5 => Department::Operations,
            _ => {
                println!("Invalid choice, try again\n");
                continue;
            }
        };

        println!("Enter name: ");
        let name = input_trimmed();

        let employee = Employee::new(name, department.clone());
        company.entry(department).or_default().push(employee);
        break;
    }
}

pub fn retrieve_all_employees_by_department(company: &mut HashMap<Department, Vec<Employee>>) {
    println!("Choose a department:");
    println!("1. Engineering");
    println!("2. Sales");
    println!("3. Marketing");
    println!("4. Finance");
    println!("5. Operations");

    println!("Enter choice: ");

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    let choice: u32 = choice.trim().parse().unwrap();

    let department = match choice {
        1 => Department::Engineering,
        2 => Department::Sales,
        3 => Department::Marketing,
        4 => Department::Finance,
        5 => Department::Operations,
        _ => {
            println!("Invalid choice.");
            return;
        }
    };

    match company.get(&department) {
        Some(employees) => {
            let mut sorted = employees.clone();
            sorted.sort_by(|a, b| a.name.cmp(&b.name));

            println!("{:?} Department Employees:", department);
            for emp in sorted {
                println!("- {}", emp.name);
            }
        }
        None => println!("No employees found in this department."),
    }
}

pub fn retrieve_all_employee_from_compnany(company: &mut HashMap<Department, Vec<Employee>>) {
    if company.is_empty() {
        println!("No employees in the company yet.");
        return;
    }

    let mut departments: Vec<&Department> = company.keys().collect();
    departments.sort();

    for department in departments {
        println!("{:?} Department:", department);

        if let Some(employees) = company.get(department) {
            let mut sorted_employees = employees.clone();
            sorted_employees.sort_by(|a, b| a.name.cmp(&b.name));

            for employee in sorted_employees {
                println!("- {}", employee.name);
            }
        }
    }
}

fn input_trimmed() -> String {
    let mut input = String::new();

    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}
