use std::collections::HashMap;
use std::io::Read;

enum TaskState { //Define enums to create a standard for states of tasks and also it is more user-friendly.
    ToDo,
    InProgress,
    Done
}

impl TaskState {
    fn enum_to_string(self) -> String { //a method for enum to convert name of enum into string
        match self{
            TaskState::ToDo => String::from("To Do"),
            TaskState::InProgress => String::from("In Progress"),
            TaskState::Done => String::from("Done")
        }
    }
}

pub trait ToDoTrait{ //trait means like an interface, it specify the methods for ToDo struct which is derived from this trait
    fn insert(&mut self, new_task:String);
    fn start(&mut self,task:String) -> Option<()>;
    fn done(&mut self,task:String) ->Option<()>;
    fn save(self) -> Result<(),std::io::Error>;
}

pub struct ToDo{
    list_map: HashMap<String,String> //HashMap is a collection which takes <Task,TaskState> like ("study hard","done"), ("work smart","todo")
}

impl ToDo{ //with 'impl' we can write some methods for struct
    pub fn new() -> Result<ToDo,std::io::Error> {
        let mut f = std::fs::OpenOptions::new().create(true).write(true).read(true).open("todo.db")?; //create a readable/writable file or open it
        let mut content:String = String::new();
        f.read_to_string(&mut content)?; //Contents of file is copied to a String "content"
        
        //This part means that for the each line in the content, do the below transformations and operations.
        //What are they? 
        //In the beginning, by writing "content.lines()" we obtain a right to walk on the lines of the our content by the help of iterator
        //Then for the each line we do:
        //Firstly, we split the 2 words in the line according to ":"("study hard : done"); then push these words("study hard", "done") into a new vector by using "collect" method
        //Secondly, using the vector "v" which we creates shortly before, we take its first and second element as &str in tuple (0,1)
        //Thirdly, convert it this tuple elements into String from &str, then by "collect" create HashMap in this case because we define list_map as HashMap.
        let list_map: HashMap<String,String> = content.lines()
            .map(|line|{line.split(" : ").collect::<Vec<&str>>()})
            .map(|v|{ (v[0],v[1]) })
            .map(|(k,t)|{ (String::from(k),String::from(t)) }).collect();

        Ok(ToDo{list_map}) //return our ToDo object in Result format
    }
}

impl ToDoTrait for ToDo{ //ToDo is derived from ToDoTrait and it must implement ToDoTrait's methods. 

    fn insert(&mut self,new_task:String){
        self.list_map.insert(new_task, TaskState::ToDo.enum_to_string());
    }

    fn start(&mut self,  task:String) -> Option<()>{ //Option returns "Some(x)" or "None"
        if let Some(task_state_map) = self.list_map.get_mut(&task){
            *task_state_map = TaskState::InProgress.enum_to_string();
            Some(())
        }
        else{
            None
        }
    }

    fn done(&mut self,task:String) -> Option<()>{
        if let Some(task_state_map) = self.list_map.get_mut(&task){
            *task_state_map = TaskState::Done.enum_to_string();
            Some(())
        }
        else{
            None
        }
    }

    fn save(self) -> Result<(),std::io::Error>{ //"Result" returns "Ok(x)" or "Err(e)" 
        let mut contents = String::new();
        for (k,t) in self.list_map{
            let each_task_state = format!("{} : {}\n",k,t); //format! is a macro and it creates a string
            contents.push_str(&each_task_state);//concatenate the "each_task_sate String" to "contents String"
        }
        std::fs::write("todo.db", contents)//Write our content String to file todo.db
    }
}