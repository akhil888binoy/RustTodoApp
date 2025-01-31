use std::io::stdin;

struct Todo{
    id : u32,
    title: String ,
    description: String,
    completed : bool
}
  

fn AddTask(todos: &mut Vec<Todo> , id : u32 , title : String , description : String , completed : bool){
    println!("Task Added {}" , id);
    let todo = Todo{
        id,
        title ,
        description,
       completed
    };
    todos.push(todo);
  }
fn main() {
   let todos : Vec<Todo> = Vec :: new()  ;   
  loop{
        println!("Enter an operation to perform \n 1.Add  \n 2.Delete \n 3.Update \n 4.View tasks ");
        let mut input : String= String :: new();
        stdin().read_line(&mut input).expect("cannot read");
        let choice : u32 = input.trim().parse().expect("not parsed");
        match choice {
            1=>{
                println!("You pressed Add Todo ")
                let id = todos.len() as u32 + 1 ;
                println!("Enter the Title");


            },
            _ => {
                println!("invalid choice")
            }
        }
  }
   


}
