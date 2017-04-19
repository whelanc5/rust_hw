use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


fn main (){

	println!("Test positive");
	jobs_ok("shifts.txt".to_string(), "employees.txt".to_string()); //checks if the assignment works, this should be true
	
	println!("Test negative");
	jobs_ok("shiftsfail.txt".to_string(), "employees.txt".to_string()); //checks if the assignment works, this should be false
	
	println!("Test negative 2");
	jobs_ok("shifts.txt".to_string(), "employeesfail.txt".to_string()); //checks if the assignment works, this should be false
	
	println!("Test negative 3");
	jobs_ok("shifts.txt".to_string(), "employeesfail2.txt".to_string()); //checks if the assignment works, this should be false
}

fn jobs_ok(assignment: String, employees: String) ->bool { //takes in name of shift and employee txt files and returns true or false 
 
	
	let s = get_input(assignment); //sets S to the list of shifts
	
	println!("The shift assignment is:  {}", s); //prints out the assignment
	
	if !check_doubles(s.clone()) { //checks to see if there are double or the wrong number of employees in the shifts
		
		println!("This shift assignment does not work, employee is assigned multiple times or wrong number of employees\n");
		return false; //returns false if check_doubles fails
	}
	
	
	let e = get_input(employees); //sets e to the list of employees followed by their skills
	
	println!("The list of employees and their skills:  {}", e); //prints out the employees and their skills

	let employees = get_employees(e); //populate employees vector. vector will hold a list of employees and their skills


	let shifts = get_shifts(s.clone());  //populate shift vector, will hold a list of shifts

	 
	let shift_and_skills = set_skills(employees, shifts); //populates shift_and_skills vector. this holds lists of shifts, each shift holds a list of employees, and each employee holds a list of strings with employee's name and skills
	
	if !check_skills(shift_and_skills){ //checks to see if the assignment works
		println!("This shift assignment does not work, the approprite skill requirements are not met\n");
		return false
	}
		println!("This shift assignment works\n");
	true
	
}
	

fn set_skills( employees: Vec<Vec<String>>,  shifts: Vec<Vec<String>>) -> Vec<Vec<Vec<String>>> { //takes in a list of employees and a list of shifts. It will return a list of shifts,
// each of those shifts will hold a list of employees on that shift, and each employee holds the employees name and its skills
	
	
	
	let mut list_temp: Vec<Vec<Vec<String>>>  = Vec::with_capacity(4); //holds list of shifts
	
	
	for x in 0..4{	//for loop for going through each shift
		let mut shift_temp: Vec<Vec<String>>  = Vec::with_capacity(4); //holds list of employees
		
		
		for z in 0..4{ // for loop for going through each employee on a shift
					 
			for y in 0..20{ //for loop to iterate through list until the employee name matches the current shift name being looked at
			
				if employees[y][0]	== shifts[x][z]{ //if employee name is equal to the shift at that location
					
					shift_temp.push(employees[y].clone()); //push employee to the list of employees on that shift
					
				}
			}	//check next employee
		} //go to next employee
		list_temp.push(shift_temp.clone()); //push the shift onto the list of shifts

	} //go to next shift

	list_temp //returns lists of shifts
	
}

fn check_skills(shift_and_skills: Vec<Vec<Vec<String>>>) ->bool { //checks to see if employees skills are correct. input should be formatted like this:
//shift_and_skills[x][y][z] x = the shift the employee is on. y = the employee's position within the shift. when z = 0 the employees name should be held, 
//when z is 1,2,3 the employee's skills in phone, computer and network should be represented respectively.
	let mut bool_val = true;
	
	for x in 0..4{ //loop for list of shifts
		
		for y in 0..4{ //loop for individual employee
			
			if y == 0{ 										//checks slot one phones
				if shift_and_skills[x][y][1].trim() == "0"{ 
				bool_val = false;
				println!("this employee is assigned to handle the phone, but is not qualified: {}", shift_and_skills[x][y][0].trim());
				}
			} else 
			if y == 1{									//checks slot two phones
				if shift_and_skills[x][y][1].trim() == "0"{
				bool_val = false;
				println!("this employee is assigned to handle the phone, but is not qualified: {}", shift_and_skills[x][y][0].trim());
				}
			} else 
			if y == 2{									//checks computer
				if shift_and_skills[x][y][2].trim() == "0"{
					bool_val = false;
					println!("this employee is assigned to handle the computer, but is not qualified: {}", shift_and_skills[x][y][0].trim());
				}
			} else
			if y == 3{ 									//checks network
				if shift_and_skills[x][y][3].trim() == "0"{
					bool_val = false;
					println!("this employee is assigned to handle the network, but is not qualified: {}", shift_and_skills[x][y][0].trim());
				}
			}
			
		}
	}
		
	
	bool_val //returns true if nothing made it go false
	
}


fn get_input(name: String) -> String{ //reads in a file name and returns a string of the content within
    // Create a path to the desired file
	
	let b = name;
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

fn check_doubles(s: String) -> bool{ //takes in string of shifts and see if there are doubles within, and ensures there are the correct number of employees assigned
	let bool_val = true;
	
	
	
	let num = s.split_whitespace().count(); //gets count of words in the list.
	
	if num != 16{ 
	
	return false;
	
	}
	
	let mut employees: Vec<String> = Vec::new();	//hold of vector of vector of employees
	
	let mut lines = s.split_whitespace(); //iterate string S by white splace
	
	let mut next = lines.next(); //holds value of the next item in s
	
	
	for _ in 0.. num {		
		employees.push(next.unwrap().to_string()); //puts next string into the employees list
		next = lines.next(); //holds value of the next item in s
	}	
	employees.sort_by(|a, b| a.cmp(b)); //sorts list	
	
	employees.dedup(); //removes doubles that are next to eachother
	
	let size = employees.len(); //int that holds the new size of employees

	
	if size != 16{ //if sized changed return false
	
		return false;  
	}
	
	bool_val //returns true if checks don't go off
	
}


fn get_shifts(s: String) -> Vec<Vec<String>>{ //takes a string of shifts and returns size 4 Vector of size 4 vectors of employees
	
	let mut shifts: Vec<Vec<String>> = Vec::new();	//holds vector of shifts
	let mut lines = s.split_whitespace(); //iterate string S by white splace
	
	for _ in 0..4{ //for loop for setting up the list of shifts
	
		let mut shifts_list: Vec<String> = Vec::new(); //holds a vector of employees on a shift
		
		for _ in 0..4{ //for loop for setting up employees
		
		shifts_list.push(lines.next().unwrap().to_string());  //push employee onto shifts list
		
		}
		shifts.push(shifts_list); //push shift onto list of shifts
	}
	
	shifts //returns shifts, 4 vector vector of employees
	
	
}

fn get_employees(s: String) -> Vec<Vec<String>>{ //takes a string of employees led by the number of employees and returns a vector of that size which holds size 4 vectors of the employee followed by their skills
	
	
	let mut lines = s.split_whitespace(); //iterate string S by white splace
	
	
	let n = lines.next().unwrap().to_string().trim().parse() //sets n to the number of employees in the list
        .expect("Please type a number!");

	
	let mut employees: Vec<Vec<String>> = Vec::new();	//vector to hold employees, the employees are a list of strings
	
	for _ in 0..n{ //for loop for going through each employee
	
		let mut skill_list: Vec<String> = Vec::new(); //vector that holds strings, to represent an employee and their skills
		
		for _ in 0..4{ //for loop for going through the employee's name and skills
		
		skill_list.push(lines.next().unwrap().to_string()); //puts the employee and their skills skills into a employee list
		}
		employees.push(skill_list); //puts the employee into the list of employees
	}
	
	employees //returns shifts, size n vector of employees. each employee is a size 4 vector of strings, first is name, 2nd is phone, 3rd is computer, 4 is network
	
}