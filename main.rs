use std::io;
use rand::Rng;
fn main() {
    let mut comp = 0 ;
    let mut hum : i32;
    io::stdin().read_line(&mut hum).expect("You entered the wrong value :(");
    if comp == hum {
        println!("same number. You are ass smart as the computer");
    } else {
        println!("try again not as smart as the computer") ;
    }
    }
    //io::stdin().read_line(&mut guess).expect("failed to readline");