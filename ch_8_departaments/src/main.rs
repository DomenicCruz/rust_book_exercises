use std::io;
use std::collections::HashMap;

const HELP_NOTE: &str = "Programm to add employee names to a department in a company.
'For example, 'Add Sally to Engineering' or 'Add Amir to Sales'
Or list employees 'list department_name' or 'list' to list all
Type 'exit' to exit
NOTE: DEMO, do not saves anything
";

fn main() {
    //create temporaty HashMap of the departments, not saved after exit
    let mut employees_of_departments: HashMap<String, Vec<String>> = HashMap::new();


    loop {
        // Read user input command
        let mut command_str: String = String::new();
        io::stdin()
            .read_line(&mut command_str)
            .expect("failed to read from stdin");
        command_str = command_str.trim().to_string().to_lowercase();
       
        //if user typed nothing
        if command_str.len() == 0 {
            println!("{}", HELP_NOTE);
            continue;
        }
         

        //Parse command_str to work with different arguments
        let mut names: Vec<String> = Vec::new();
        let mut departments: Vec<String> = Vec::new();
        let mut command_words = command_str.split_whitespace();
        let cmd_name = command_words.next().unwrap();
        let command_words = command_words.collect::<Vec<_>>();
       
        //  Command Handling  //

        //  if user chose command exit
        if cmd_name == "exit"{
            println!("bye!");
            break;
        }

        
        //  if user chose command add
        if cmd_name == "add" {
            for (idx, word) in command_words.iter().enumerate() {
                //if encountered keyword 'to', add words starting from next to the departament s
                if word.to_string() == "to" && command_words.len() > idx{
                    //departments = command_words[idx + 1..].to_vec();
                    for departament in &command_words[idx + 1..] {
                        departments.push(departament.to_string());
                    }
                    break;//Stop adding words, exit loop
                }
                //Add every word as a name
                names.push(word.to_string());
            }
            if names.len() == 0 || departments.len() == 0 {
                println!("Input names of employees and departments"); 
                continue;//get new user input
            }
            //println!("names: {:?}", names);
            //println!("departments: {:?}", departments);
            add_employees(&mut employees_of_departments, departments, names);
            //println!("employees_of_departments: {:?}", employees_of_departments);
        } else if cmd_name == "list" {
            //  if user chose command list
            for departament in &command_words[..] {
                departments.push(departament.to_string());
            }
            //println!("departments: {:?}", departments);
            if departments.len() == 0 {
                //list all employee names across all departments
                for (dep_name, empl_in_dep) in &employees_of_departments  {
                   println!("{} departament employees:", str_capitalise(dep_name));
                   print_names_vec(empl_in_dep); 
                }
            }else {
                //list employees for some of the departments
                list_employees(&employees_of_departments, departments); 
            }
        } else {
            //command do not exists
            println!("{}", HELP_NOTE);
            continue;
        }
    //end of outer loop
    println!("+-+");
    }
}


fn list_employees(empls_of_deps: &HashMap<String, Vec<String>>, deps: Vec<String>) {
    for dep in &deps {
        println!("{} departament employees:", str_capitalise(dep));
        match empls_of_deps.get(dep) {
            Some(list_of_empls) => print_names_vec(list_of_empls),
            None => (),
        }
    }
}

fn print_names_vec(str_vec: &Vec<String>) {
   for string in str_vec {
       println!("{}", str_capitalise(string));
   } 
}

fn str_capitalise(s: &str) -> String {
  format!("{}{}", (&s[..1].to_string()).to_uppercase(), &s[1..])
}

fn add_employees(empls_of_deps: &mut HashMap<String, Vec<String>>, deps: Vec<String>, empls: Vec<String>) {
    for dep in &deps {
       //Create or use existing list ow employees
       let empl_list = empls_of_deps.entry(dep.to_string()).or_insert(Vec::new());
       for empl in &empls {
         empl_list.push(empl.to_string());
       }
    }
}

