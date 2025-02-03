use std::{io::{stdin, Read}, pin};
#[derive(Debug)] 

struct Todo{
    id : u32,
    title: String ,
    description: String,
    completed : bool
}


fn getinput(prompt : &str) -> Option<String> {
    println!("{}", prompt);
    let mut input = String :: new();
    stdin().read_line(&mut input).expect("cant read");
    return Some(input.trim().to_string());
}


fn AddTask(todos: &mut Vec<Todo> , id : u32 , title : String , description : String , completed : bool){
   let todo = Todo{
        id,
        title ,
        description,
        completed
    };
    todos.push(todo);
    println!("Todo added Id : {}",id);
}


  fn UpdateTask(todos: &mut Vec<Todo> , id : u32 , newtitle : String , newdescription : String , newcompleted : bool){

     if let  Some(todo) = todos.iter_mut().find( |t| t.id == id) {
        todo.title = newtitle ;
        todo.description = newdescription ;
        todo.completed = newcompleted ;
        println!("Todo is updated for TodoID : {}" , id);
    }else {
        println!("Todo is not updated : {}" ,id);
    }
  }


  fn DeleteTask(todos: &mut Vec<Todo> , id : u32){
    if let Some(index) = todos.iter_mut().position(|t| t.id == id) {
        todos.remove(index );
    }else {
        println!("Cannot Delete Todo : {}" , id);
    }
  }

fn AllTodos (todos : &mut Vec<Todo>){
    println!("These are the tasks");
    for todo in todos.iter(){
        println!("{:?}" , todo);
    }
}

fn MarkComplete(todos : &mut Vec<Todo> , id : u32){

   if let Some(todo) = todos.iter_mut().find(|t| t.id == id){
        todo.completed = true;
        println!("Marked Completed For Todo:{}" , id);
    }else {
        println!("Cannot Mark Completed: {}" , id);
    }
}
  

fn main() {
    let mut todos : Vec<Todo> = Vec :: new();   
    loop{
        println!("Enter an operation to perform \n 1.Add  \n 2.Delete \n 3.Update \n 4.View tasks \n 5.Mark Completed \n 7.Exit");
        let mut input : String= String :: new();
        stdin().read_line(&mut input).expect("cannot read");
        let choice : u32 = input.trim().parse().expect("not parsed");
        match choice {
            1=>{
                AllTodos(&mut todos);
                println!("You pressed Add Todo ");
                let id = todos.len() as u32 + 1 ;
                let title = match getinput("Enter the title"){
                    Some(t)=> t,
                    None =>{
                        print!("add valid input");
                        continue
                    }
                };
                let description = match getinput("Enter the Description") {
                    Some(t)=> t,
                    None =>{
                        print!("add valid input");
                        continue
                    }
                };
               let completed = false;
               AddTask(&mut todos, id, title, description, completed);
               println!("Task Added Id : {}", id);              
            },
            2=>{
                AllTodos(&mut todos);
                let mut id = String :: new();
                stdin().read_line(&mut id).expect("cannot read");
                let id: u32 = id.trim().parse().expect("cannot parse");
                DeleteTask(&mut todos, id);
            },
            3=>{
                AllTodos(&mut todos);
                println!("Enter Id of todo");
                let mut id : String = String:: new();
                stdin().read_line(&mut id).expect("Cannot read input ");
                let id : u32 = id.trim().parse().expect("cannot parse");
                let title = match getinput("Give new title") {
                    Some(t)=> t,
                    None => {
                        println!("Error hit");
                        continue
                    }
                };
                let description = match getinput("Give updated description") {
                    Some(t)=>t,
                    None =>{
                        println!("Error hit");
                        continue
                    }
                };
                UpdateTask(&mut todos, id , title  , description  , false );

            },
            4=>{
               AllTodos(&mut todos);
            },
            5=>{
                println!("Enter Id of todo to be completed");
                let mut id = String :: new();
                stdin().read_line(&mut id ).expect("Cannot read");
                let id = id.trim().parse().expect("cannot parse");
                MarkComplete(&mut todos, id);
             },
            7=>{
                println!("Bye Bye");
                return;
            },
            _ => {
                println!("invalid choice");
                continue
            }
        }
  }

//   let mut s = 10 ;
//   let s1 = &mut s ;
//   *s1 = 11 ;
//   println!("{}", s)


}


