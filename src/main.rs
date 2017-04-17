use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io;

fn main (){

	//println!("Test positive");
	//jobs_ok("shifts.txt".to_string(), "employees.txt".to_string());
	
	//println!("Test negative");
	//jobs_ok("shiftsfail.txt".to_string(), "employees.txt".to_string());
	
	//println!("Test negative 2");
	//jobs_ok("shifts.txt".to_string(), "employeesfail.txt".to_string());
	
	
	println!("Test a file? y/n");
	
let mut input = String::new();
	match io::stdin().read_line(&mut input) {
    Ok(n) => {  }
    Err(error) => println!("error: {}", error),
	}
	if (input.trim() == "y"){
		println!("Enter the name of the shifts text file");
		let mut sinput = String::new();
		match io::stdin().read_line(&mut sinput) {
		Ok(n) => {  }
		Err(error) => println!("error: {}", error),
		}
		println!("Enter the name of the employees text file");
		let mut einput = String::new();
		match io::stdin().read_line(&mut einput) {
		Ok(n) => {  }
		Err(error) => println!("error: {}", error),
		}
		jobs_ok(sinput.trim().to_string(),einput.trim().to_string());
	}
}

fn jobs_ok(Assignment: String, Employees: String) ->bool { //takes in name of shift and employee txt files and returns true or false 
 
	let mut s = String::new();
	s = get_input(Assignment);
	
	let mut doubles = true;
	doubles = check_doubles(s.clone());
	if !doubles {
		
		println!("This shift assignment does not work, employee is assigned multiple times or wrong number of employees");
		return false;
	}
	
	let mut e = String::new();
	e = get_input(Employees);
	
	let mut employees: Vec<Vec<String>> = Vec::new(); //list for holding employees and their skills
	employees = get_employees(e); //populate employee vector

	
	let mut shifts: Vec<Vec<String>> = Vec::new(); //list for holding list of shifts
	shifts = get_shifts(s.clone()); 

	let mut shift_and_skills: Vec<Vec<Vec<String>>> = Vec::new(); //holds lists of shifts, each shift holds a list of employees, and each employee holds a list of strings with employee's name and skills
	shift_and_skills = set_skills(employees, shifts);
	
	if !check_skills(shift_and_skills){
		println!("This shift assignment does not work, the approprite skill requirements are not met");
		return false
	}
		println!("This shift assignment works");
	true
	
}
	

fn set_skills( mut employees: Vec<Vec<String>>,  shifts: Vec<Vec<String>>) -> Vec<Vec<Vec<String>>> { //makes a 4x4 list of lists of strings
	
	
	//let mut shift_and_skills: Vec<Vec<Vec<Vec<String>>>>  = Vec::with_capacity(4);
	let mut list_temp: Vec<Vec<Vec<String>>>  = Vec::with_capacity(4);
	
	
	for x in 0..4{	
		let mut shift_temp: Vec<Vec<String>>  = Vec::with_capacity(4);
		let mut shift: Vec<String> = Vec::new();
		
		for z in 0..4{
					
			for y in 0..20{
				if employees[y][0]	== shifts[x][z]{
					
					shift_temp.push(employees[y].clone());
					
				}
			}	
		}
		list_temp.push(shift_temp.clone());

	}

	list_temp //returns lists
	
}

fn check_skills(shift_and_skills: Vec<Vec<Vec<String>>>) ->bool { //checks to see if employees skills are correct
	let mut bool_val = true;
	
	for x in 0..4{ //loop for list of shifts
		
		for y in 0..4{ //loop for individual employee
			//println!("{}", shift_and_skills[x][y][0].trim());
			if y == 0{ 										//checks slot one phones
				if shift_and_skills[x][y][1].trim() == "0"{ 
				bool_val = false;
			//	println!("{}", shift_and_skills[x][y][1].trim());
				}
			} else 
			if y == 1{									//checks slot two phones
				if shift_and_skills[x][y][1].trim() == "0"{
				bool_val = false;
				println!("{}", shift_and_skills[x][y][1].trim());
				}
			} else 
			if y == 2{									//checks computer
				if shift_and_skills[x][y][2].trim() == "0"{
					bool_val = false;
					println!("{}", shift_and_skills[x][y][2].trim());
				}
			} else
			if y == 3{ 									//checks network
				if shift_and_skills[x][y][3].trim() == "0"{
					bool_val = false;
					println!("{}", shift_and_skills[x][y][3].trim());
				}
			}
			
		}
	}
		
	
	bool_val //returns true if nothing made it go false
	
}


fn get_input(name: String) -> String{ //reads in a file name and returns a string of the content within
    // Create a path to the desired file
	let mut b = String::new();
	b = name;
    let path = Path::new(&b);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   why.description()),
        Ok(_) => print!("{}", ""),
    }
	
	
	 s //return string of value from file

}

fn check_doubles(s: String) -> bool{ //takes a string of shifts and returns size 4 Vector of size 4 vectors of employees
	let mut bool_val = true;
	
	let mut lines = s.split_whitespace(); //iterate string S by white splace
	let mut next = lines.next();
	let mut employees: Vec<String> = Vec::new();	//hold of vector of vector of employees
	let num = s.split_whitespace().count(); //gets count of words in the list.
	if num != 16{
	return false;
	}
	for _ in 0.. num {		
		employees.push(next.unwrap().to_string());
		next = lines.next();
	}	
	employees.sort_by(|a, b| a.cmp(b)); //sorts list	
	
	employees.dedup(); //removes doubles that are next to eachother
	let size = employees.len();

	
	if size != 16{
		return false;
	}
	
	bool_val
	
}


fn get_shifts(s: String) -> Vec<Vec<String>>{ //takes a string of shifts and returns size 4 Vector of size 4 vectors of employees
	
	let mut shifts: Vec<Vec<String>> = Vec::new();	//hold of vector of vector of employees
	let mut lines = s.split_whitespace(); //iterate string S by white splace
	for _ in 0..4{
		let mut shifts_list: Vec<String> = Vec::new(); //holds vector a employees
		for _ in 0..4{
		shifts_list.push(lines.next().unwrap().to_string());
		}
		shifts.push(shifts_list);
	}
	
	shifts //returns shifts, 4 vector vector of employees
	
	
}

fn get_employees(s: String) -> Vec<Vec<String>>{ //takes a string of employees led by the number of employees and returns a vector of that size which holds size 4 vectors of the employee followed by their skills
	
	let mut employees: Vec<Vec<String>> = Vec::new();	//hold of vector of vector of employees
	let mut lines = s.split_whitespace(); //iterate string S by white splace
	
	//let mut n = 0;
	let n = lines.next().unwrap().to_string().trim().parse() //sets n to the number of employees in the list
        .expect("Please type a number!");
	//println!("{}", n);
	
	for _ in 0..n{
		let mut skill_list: Vec<String> = Vec::new(); //holds vector of employees
		for _ in 0..4{
		skill_list.push(lines.next().unwrap().to_string()); //puts the employee skills into a employee list
		}
		employees.push(skill_list); //puts the employee into the list of employees
	}
	
	employees //returns shifts, size n vector of employees. each employee is a size 4 vector of strings, first is name, 2nd is phone, 3rd is computer, 4 is network
	
}