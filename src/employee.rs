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

pub fn add_employee(department: Department, company: &mut HashMap<Department, Vec<Employee>>) {
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

        match choice {
            1 => {
                println!("Enter name: ");
                let name = input_trimmed();
                let employee = Employee::new(name, department.clone());

                company.entry(department).or_default().push(employee);
            }
            2 => {
                println!("Enter name: ");
                let name = input_trimmed();
                let employee = Employee::new(name, department.clone());

                company.entry(department).or_default().push(employee);
            }
            3 => {
                println!("Enter name: ");
                let name = input_trimmed();
                let employee = Employee::new(name, department.clone());

                company.entry(department).or_default().push(employee);
            }
            4 => {
                println!("Enter name: ");
                let name = input_trimmed();
                let employee = Employee::new(name, department.clone());

                company.entry(department).or_default().push(employee);
            }
            5 => {
                println!("Enter name: ");
                let name = input_trimmed();
                let employee = Employee::new(name, department.clone());

                company.entry(department).or_default().push(employee);
            }
            _ => {
                println!("Invalid choice, try again");
                continue;
            }
        }
    }
}

pub fn retrieve_all_employee_by_department() {}

pub fn retrieve_all_employee_from_compnany() {}

fn input_trimmed() -> String {
    let mut input = String::new();

    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}
