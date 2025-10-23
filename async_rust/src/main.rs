use async_std::{fs::File, io::{self, ReadExt}, task::{self, block_on}};



async fn read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    Ok(contents)
}


fn main() {
    let task = task::spawn(async {
        let result = read_file("cargo.lock").await;
        match result {
            Ok(str) => println!("{}", str),
            Err(err) => panic!("{}", err)
        };
    });
    
    println!("Task has started");
    block_on(task);
    println!("Stopped task");
}
