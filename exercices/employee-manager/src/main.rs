// Using a hash map and vectors,
// create a text interface to allow a user to add employee names to a department in a company.
// For example, “Add Sally to Engineering” or “Add Amir to Sales.”
// Then let the user retrieve a list of all people in a department or all people in the company by department,
// sorted alphabetically.
use std::io;
use std::collections::HashMap;

fn user_input(ouput: &str) -> String {
    println!("{}", ouput);

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_string()
}

fn main() {
    let mut employees_by_department: HashMap< String, Vec<String> >= HashMap::new();

    loop {
        let employee_name = user_input("What's the name of the new employee ?");
        let department = user_input("Where will she/he work?");

        employees_by_department.entry(department).or_insert(Vec::new()).push(employee_name);


        let add_another = user_input("Do you wish to add another employee? [y]");

        if add_another != "y" { break };
    };

    for (department, employees) in employees_by_department {
        println!("{} department ({}):", department, employees.len());
        for employee in employees {
            println!("- {}", employee);
        }
    }
}
