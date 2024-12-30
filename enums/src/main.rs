enum Message {
    _Quit,
    ChangeColor(i32, i32, i32),
    Move { x: i32, y: i32 },
    Write(String),
}
 fn chang(color: Message){
    match color {
        Message::_Quit => println!("Quit"),
        Message::ChangeColor(r, g, b) =>{
            let a = 2 + r;
            let d:f32 = 0.5 + g as f32;
            let c = 2 + b;
        
          println!("Change color to RGB({}, {}, {})", a, d, c);
        },
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(ref s) => println!("Write message: {}", s),
    }    

 }

    fn main() {
        let msg = Message::Move { x: 10, y: 20 };
        let a = Message::Write("text of enums".to_string());
        let colors = Message:: ChangeColor(12,10,11);
        match msg {
            Message::_Quit => println!("Quit"),
            Message::ChangeColor(r, g, b) => println!("Change color to RGB({}, {}, {})", r, g, b),
            Message::Move { x, y } => println!("Move to ({}, {})", x, y),
            Message::Write(ref s) => println!("Write message: {}", s),
        }
        match a {
            Message::_Quit => println!("Quit"),
            Message::ChangeColor(r, g, b) => println!("Change color to RGB({}, {}, {})", r, g, b),
            Message::Move { x, y } => println!("Move to ({}, {})", x, y),
            Message::Write(ref s) => println!("Write message: {}", s),
        }
        chang(colors);
    }

