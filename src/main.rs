use std::io;
mod department;
mod employee;
use crate::department::Department;
use crate::employee::{
    add_employee, retrieve_all_employee_from_compnany, retrieve_all_employees_by_department,
    Employee,
};
use std::collections::HashMap;

fn main() {
    // Using a hash map and vectors, create a text interface to allow a
    // user to add employee names to a department in a company; for
    // example, “Add Sally to Engineering” or “Add Amir to Sales.” Then
    // let the user retrieve a list of all people in a department or all
    // people in the company by department, sorted alphabetically.

    let mut company: HashMap<Department, Vec<Employee>> = HashMap::new();
    println!("Welcome to the Human Resource Department");

    loop {
        println!("What would you like to do?");
        println!("1. Add employee");
        println!("2. Retrieve employees by department");
        println!("3. Retrieve all employees");
        println!("4. Exit");

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
            1 => add_employee(&mut company),
            2 => retrieve_all_employees_by_department(&mut company),
            3 => retrieve_all_employee_from_compnany(&mut company),
            4 => {
                println!("Goodbye! 👋");
                break;
            }
            _ => {
                println!("Invalid choice, try again.");
                continue;
            }
        };
    }
}
