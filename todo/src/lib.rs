mod list {
    #[derive(Debug)]
    pub struct Tasks {
        pub item: String
    }
    
    
    
    mod things_completed {
        fn remove_activity() {}
        fn redo() {}
    }
    
}

mod things_todo;

fn add_task() {
    let task = list::Tasks{ item: String::from("task1")};
    things_todo::add_activity();
}