use std::collections::HashMap;

fn main() {
    let mut departments: HashMap<String, Vec<&str>> = HashMap::new();
    create_department(&mut departments); 
    print_hashmap(&departments);  
}

fn create_department(hm:&mut HashMap<String, Vec<&str>>) {
    println!("Enter name of department you would like to create.."); 
    let mut department = String::from("");
    std::io::stdin().read_line(&mut department).unwrap();
    hm.insert(String::from(department), Vec::new()); 
}

fn add_employee(hm: &mut HashMap<String, Vec<&str>>) {
    let mut name = String::from("");
    let mut department = String::from(""); 
    println!("Enter name of employee.."); 
    std::io::stdin().read_line(&mut name).unwrap(); 
    println!("Enter department to add employee to.."); 
    std::io::stdin().read_line(&mut department).unwrap(); 
}   

fn print_hashmap(hm:&HashMap<String, Vec<&str>>) {
    for(key, value) in hm {
        println!("{}: {:?}", key, value); 

    }
}
