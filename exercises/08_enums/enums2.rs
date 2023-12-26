// enums2.rs
//
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a
// hint.



#[derive(Debug)]
enum Message {
    // TODO: define the different variants used below
    Move{       //this is how to define a enum struct variant 
        x: u32,
        y: u32,
    },
    Echo(String),       //this is how to define a enum tuple variant 
    ChangeColor(u8,u8,u8), 
    Quit                    //basic enum variant 
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
