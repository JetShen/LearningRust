struct Task{
    name: String,
    description: String,
    priority: u8,
    done: bool,
}

impl Task{
    fn new(name: String, description: String, priority: u8) -> Task{ // u8 is a number from 0 to 255
        Task{
            name,
            description,
            priority,
            done: false,
        }
    }
    fn done(&mut self){
        self.done = true;
    }
    fn undone(&mut self){
        self.done = false;
    }
    fn print(&self){
        println!("Name: {}", self.name);
        println!("Description: {}", self.description);
        println!("Priority: {}", self.priority);
        println!("Done: {}", self.done);
    }
}

fn createtask() -> Task{
    println!("Please enter the name of the task");
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).expect("Failed to read line");
    let name = name.trim().to_string();

    println!("Please enter the description of the task");
    let mut description = String::new();
    std::io::stdin().read_line(&mut description).expect("Failed to read line");
    let description = description.trim().to_string();

    println!("Please enter the priority of the task");
    let mut priority = String::new();
    std::io::stdin().read_line(&mut priority).expect("Failed to read line");
    let priority: u8 = priority.trim().parse().expect("Priority entered was not a number");

    Task::new(name, description, priority)
}

fn menu() -> u8 {
    println!("Please enter the number of the choice you want to do");
    println!("1. Add a task");
    println!("2. Delete a task");
    println!("3. Mark a task as done");
    println!("4. Mark a task as undone");
    println!("5. Print all the task");
    println!("6. Exit");
    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).expect("Failed to read line");
    let choice: u8 = choice.trim().parse().expect("choice entered was not a number");
    choice
}

fn main(){
    // let listoftask = [Task; 1] = Task{
    //     name: String::new(),
    //     description: String::new(),
    //     priority: u8,
    //     done: bool,
    // }; // make a list of max 10 task where you cant add  task or delete task

    let mut tasks: Vec<Task> = Vec::new(); // make a list of task where you can add or delete task

    loop{
        let choice = menu();
        if choice == 1{
            tasks.push(createtask());
        }
        else if choice == 2{
            println!("Please enter the number of the task you want to delete");
            let mut index = String::new();
            std::io::stdin().read_line(&mut index).expect("Failed to read line");
            let index: usize = index.trim().parse().expect("Index entered was not a number");
            tasks.remove(index);
        }
        else if choice == 3{
            println!("Please enter the number of the task you want to mark as done");
            let mut index = String::new();
            std::io::stdin().read_line(&mut index).expect("Failed to read line");
            let index: usize = index.trim().parse().expect("Index entered was not a number");
            tasks[index].done();
        }
        else if choice == 4{
            println!("Please enter the number of the task you want to mark as undone");
            let mut index = String::new();
            std::io::stdin().read_line(&mut index).expect("Failed to read line");
            let index: usize = index.trim().parse().expect("Index entered was not a number");
            tasks[index].undone();
        }
        else if choice == 5{
            for task in tasks.iter(){
                println!("-------------------------");
                task.print();
            }
            println!("-------------------------");
        }
        else if choice == 6{
            break;
        }
    }
}