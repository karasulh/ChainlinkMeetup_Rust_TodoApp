use chainlinkmeetup_todoapp::{ToDo, ToDoTrait};

//cargo run "add" "work smart"
//cargo run "done" "work smart"

fn main() {

    let args:Vec<String> = std::env::args().collect(); //it takes the inputs when we run this code from terminal
    if args.len() < 3{ //cargo run(0) "done"(1) "work smart"(2)
        panic!("Too less arguments passed!");
    }
    let action = args[1].clone();
    let task = args[2].clone();

    let mut todolist = ToDo::new().expect("Error in creating a ToDoApp"); //expect is used to print error if we meet an error.

    if action == "add" {
        todolist.insert(task);
        match todolist.save() { //todolist.save() returns "Result" type
            Ok(_) => println!("New task is added to ToDo List"), //Ok(_)=> "_" means that we dont care the inside of "Ok". "Ok" result is enough for us.
            Err(e) => println!("Error: {}",e)  
        }
    }
    else if action == "start" {

        //let result:Option<()> = todolist.start(task);
        //match result {
        //    None => ...,
        //    Some(x) => ... }
        match todolist.start(task) { //It has the same meaning with above lines
            None => println!("There is no such task in ToDo List"),
            Some(_) => {
                match todolist.save() {
                    Ok(_) => println!("Task is in progress"),
                    Err(e) => println!("Error {}",e)
                }
            }    
        }
    }
    else if action == "done" {
        match todolist.done(task) { //todolist.done() returns "Option" type
            None => println!("There is no such task in ToDo List"),
            Some(_) => {
                match todolist.save() {
                    Ok(_) => println!("Task is done"),
                    Err(e) => println!("Error: {}",e)
                }
            }
        }
    }
    else{
        println!("Wrong actions are given");
    }
}
