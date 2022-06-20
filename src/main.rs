use std::io;
use std::collections::HashMap;
#[derive(Debug,Clone)]
pub struct Student{
    name : String,
    mark :i32,
}

pub struct School {
    students: HashMap<String,i32>,
}

impl School {
    fn new() -> Self {
        Self {students: HashMap::new() }
    }
    fn add(&mut self, student : Student ) {
        self.students.insert(student.name.to_string(), student.mark);
    }

    pub fn enumtMark(&self) -> Vec<i32> {
        let grades = self.students.values();
        let mut result:Vec<i32> = Vec::new();
        for x in grades.into_iter(){
            result.push(*x);
        }
        result.sort();
        result.dedup();
        result
    }

    pub fn enumStudent(&self, mark: i32) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        for (k,v) in &self.students {
            if *v == mark {
                result.push(k.to_owned());
            }
        }
        result.sort();
        result
    }

}

mod manager {
    use crate::*;
    pub fn add_student (students : &mut School) {
        println!("Enter name of Student");
        let name = match get_input() {
            Some(input) => input,
            None => return,
        };

        let mark = match get_int() {
            Some(input) => input,
            None => return,
        };
        let student = Student{name,mark};
        students.add(student)

    }
    pub fn enum_Mark(students : &mut School){
        println!("{:#?}", students.enumtMark());
    }

    pub fn enum_Student(students : &mut School){
        let mark = match get_int() {
            Some(input) => input,
            None => return,
        };
        println!("{:#?}", students.enumStudent(mark));
    }

}

fn get_input() -> Option<String>{
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Let try again");
    }
    let input = buffer.trim().to_owned();
    if input == ""{
        None
    } else {
        Some(input)
    }
}

fn get_int() -> Option<i32> {
    let input = match get_input() {
        Some(input) => input,
        None => return None,
    };
    let parse_input: Result<i32, _> =input.parse();
    match parse_input {
        Ok(input) => Some(input),
        Err(_) => None,
    }

}

enum Manager {
    addStudent,
    enumMark,
    enumStudent,
}

impl Manager {
    fn show() {
        println!("1. addStudent ");
        println!("2. enumtMark ");
        println!("3. enumStudent ");
        println!("Please choice :") ;        
    }
    fn choice(input : &str) -> Option<Manager> {
        match input {
            "1" => Some(Manager::addStudent),
            "2" => Some(Manager::enumMark),
            "3" => Some(Manager::enumStudent),
            _ => None,
        }
    }
}

fn main() {
    let mut students =School::new();
    loop {
        Manager::show();
        let input = get_input().expect("Please enter your data:");
        match Manager::choice(&input.as_str()) {
            Some(Manager::addStudent) => manager::add_student(&mut students),
            Some(Manager::enumMark) => manager::enum_Mark(&mut students),
            Some(Manager::enumStudent) => manager::enum_Student(&mut students),
            None => return,
        }
    }   
}
