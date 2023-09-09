/* //Paste it to Rust Playground and uncomment its beginning and end 

use std::collections::HashMap;
use std::io::Read;

enum TaskState{
    ToDo,
    InProgress,
    Done
}

impl TaskState{
    fn enum_to_string(self) -> String{
        match self{
            TaskState::ToDo => "To Do".to_string(),
            TaskState::InProgress => "In Progress".to_string(),
            TaskState::Done => "Done".to_string()
        }
    }
}

pub trait ToDoTrait{
    fn insert(&mut self,new_task:String);
    fn start(&mut self,task:String) -> Option<()>;
    fn done(&mut self,task:String) -> Option<()>;
    fn save(self) -> Result<(),std::io::Error>;
}

struct ToDo{
    list_map: HashMap<String,String>
}

impl ToDo{
    fn new() -> Result<Self,std::io::Error> {
        let mut f = std::fs::OpenOptions::new().create(true).write(true).read(true).open("todo.db")?;
        let mut content:String = String::new();
        f.read_to_string(&mut content)?;
        let list_map: HashMap<String,String> = content.lines()
            .map( |line| {line.split(":").collect::<Vec<&str>>() } )
            .map( |v| { (String::from(v[0]),String::from(v[1])) } )
            .collect();
        Ok(ToDo{list_map})
    }
}

impl ToDoTrait for ToDo{
    fn insert(&mut self, new_task:String){
        self.list_map.insert(new_task, TaskState::ToDo.enum_to_string());
    }
    
    fn start(&mut self, task:String) -> Option<()>{
        if let Some(task_state_map) = self.list_map.get_mut(&task){
            *task_state_map = TaskState::InProgress.enum_to_string();
            Some(())
        }
        else{
            None
        }
    }
    
    fn done(&mut self, task:String) -> Option<()>{
        if let Some(task_state_map) = self.list_map.get_mut(&task){
            *task_state_map = TaskState::Done.enum_to_string();
            Some(())
        }
        else{
            None
        }
    }
    
    fn save(self) -> Result<(),std::io::Error> {
        let mut contents = String::new();
        for (k,v) in self.list_map{
            let each_task_state = format!("{} : {}\n",k,v);
            contents.push_str(&each_task_state);
        }
        std::fs::write("todo.db",contents)
    }
    
}

fn main(){
    
    //let args:Vec<String> = std::env::args().collect();
    let args:Vec<String> = vec!["run".to_string(), "add".to_string(), "study more".to_string()]; //cargo run "study more" "todo"
    if args.len() < 3 {
        panic!("Too less arguments passed");
    }
    
    let action = args[1].clone();
    let task = args[2].clone();
    
    let mut todolist = ToDo::new().expect("Error in creating a ToDoApp");
    
    if action == "add" {
        todolist.insert(task);
        match todolist.save(){
            Ok(_) => println!("Task is added to To Do List"),
            Err(e) => println!("Error {}",e)
        }
    }
    
    else if action == "start" {
        
        //let result:Option<()> = todolist.start(task);
        //match result {}
        match todolist.start(task) {
            None => println!("There is no such task in ToDo List"),
            Some(_) => match todolist.save(){
                Ok(_) => println!("Task is in progress"),
                Err(e) => println!("Error {}",e)
            }
        }
    }
    
    else if action == "done" {
        match todolist.done(task) {
            None => println!("There is no such task in ToDo List"),
            Some(_) => match todolist.save(){
                Ok(_) => println!("Task is done"),
                Err(e) => println!("Error {}",e)
            }
        }
    }
    
    else{
        println!("Wrong action");
    }
    
}

*/
