use std::collections::HashMap;
use std::collections::hash_map::Entry; 

fn main() {
    let mut departments: HashMap<String, Vec<String>> = HashMap::new(); 
    let mut option: i32; 
    let mut run = true; 

    while run {
        print_menu(); 
        option = get_option(); 
        match option{
            1 => create_department(&mut departments), 
            2 => get_info(&mut departments),
            3 => display_department(& departments), 
            4 => print_company(& departments), 
            5 => run = false, 
            _ => println!("Invalid option select!"), 
        }
    }
}

fn print_menu() {
    println!("  Please enter option number! "); 
    println!("------------------------------");
    println!("|    1. Create Department    |");
    println!("|    2. Add Employee         |");
    println!("|    3. Display Department   |");
    println!("|    4. Display Company      |"); 
    println!("|    5. Exit Program         |"); 
    println!("------------------------------"); 
}

fn get_option() -> i32 {
    let mut read_in = String::new();  
    std::io::stdin().read_line(&mut read_in).expect("Failed to read line");
    let option: i32 = read_in.trim().parse().expect("Input not an integer"); 
    return option; 
}

fn create_department(hm:&mut HashMap<String, Vec<String>>) {
    println!("Enter name of department you would like to create.."); 
    let mut department = String::from("");
    std::io::stdin().read_line(&mut department).unwrap();
    let department = department.trim().to_string();
    hm.insert(department, Vec::new());
    println!("New Department Created!"); 
}

fn  get_info(hm: &mut HashMap<String, Vec<String>>) {
    let mut name = String::from("");
    let mut department = String::from(""); 
    println!("Enter name of employee.."); 
    std::io::stdin().read_line(&mut name).unwrap();  
    println!("Enter department to add employee to.."); 
    std::io::stdin().read_line(&mut department).unwrap(); 
    let name = name.trim().to_string(); 
    let department = department.trim().to_string(); 
    add_employee(hm, department, name); 
}

fn add_employee(hm:&mut HashMap<String, Vec<String>>, department: String, name: String) {
    let key = String::from(department);
    match hm.entry(key.clone()) {
        Entry::Vacant(..) => { println!("This department does not exist!!"); }
        Entry::Occupied(mut e) => { 
            e.get_mut().push(name.clone()); 
            println!("Employee successfully added!"); 
        }
    }
}

fn display_department(hm: &HashMap<String, Vec<String>>) {
    let mut department = String::from(""); 
    println!("Enter the department to display..");  
    std::io::stdin().read_line(&mut department).unwrap(); 
    let department = department.trim();  

    match hm.get(department) {
        Some(employees) => {
            println!("{} Department:", department); 
            for employee in employees {
                println! (" - {}", employee); 
            }
        },
        None => println!("No '{}' department found!", department), 
    }
}

fn print_company(hm:&HashMap<String, Vec<String>>) {
    for(key, value) in hm {
        println!("{}: {:?}", key, value); 
    }
}
