use std::io;
use std::collections::HashMap;

pub fn menu() {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let mut emps: Vec<String> = Vec::new();
    let mut deps: Vec<String> = Vec::new();

    println!("Welcome to the records text interface!");

    let mut addnew = "Y".to_string();

    while addnew == "Y" {
        let mut empname = String::new();
        let mut deptname = String::new();

        println!("Enter the name of the employee you wish to add to the records:");
        io::stdin()
            .read_line(&mut empname)
            .expect("Please enter a valid name.");

        println!("Enter the name of the department of the employee:");
        io::stdin()
            .read_line(&mut deptname)
            .expect("Please enter a valid department name.");

        println!("Your command is: Add {} to {}.", empname.trim().to_string(), deptname.trim().to_string());

        emps.push(empname.trim().to_string());
        deps.push(deptname.trim().to_string());

        println!("Do you want to add more employees? [Y/n]");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Please input only Y or n.");

        addnew = input.trim().to_string();
    }

    for (i, dept) in deps.iter().enumerate() {
        let emp = &emps[i];

        map.entry(dept.to_string())
            .or_insert_with(Vec::new)
            .push(emp.to_string());
    }

    let mut i = String::new();

    println!("Input 1 if you wish to list all employees of a specific department.");
    println!("Input 2 if you wish to list all employees of the company by department, sorted alphabetically.");
    println!("Enter your input:");

    io::stdin()
        .read_line(&mut i)
        .expect("Please input a number.");

    let i: u32 = match i.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    if i == 1 {
        let mut searchdept = String::new();

        println!("Enter the name of the department you wish to list all employees of:");

        io::stdin()
            .read_line(&mut searchdept)
            .expect("Please enter a valid department name.");

        println!("The employees in the chosen department are:");

        for (key, value) in &map {
            if key == &searchdept.trim().to_string() {
                println!("{key} - \n\t {value:?}");
            }
        }
    } else if i == 2 {
        let mut sorted_deps: Vec<String> = map.keys().cloned().collect();
        sorted_deps.sort();
        println!("All the employees in the company per department sorted alphabetically are:");
        for dept in sorted_deps {
            if let Some(emp) = map.get(&dept) {
                let mut sorted_emps = emp.clone();
                sorted_emps.sort();
                println!("{} - \n\t {:?}", dept, sorted_emps);
            }
        }
    } else {
        println!("Please input either 1 or 2.");
    }
}
