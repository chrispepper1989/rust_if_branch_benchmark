extern crate stopwatch;
use stopwatch::{Stopwatch};
use rand::Rng;

//string messages
const USER_MESSAGES: &'static [&'static str] = &["Sorry no beer for you", "Not in the USA", "I will need to check you", "sure you can drink!"];

fn main() {
    custom_benchmark();
}
fn custom_benchmark()
{
    let mut rng = rand::thread_rng();
    let mut _age = rng.gen_range(17,35);    
    let mut sw;    
    let mut total = 0;
    let runs = 10000000;

    for  _i in 1..runs
    {
        _age = rng.gen_range(17,35);
        
        sw = Stopwatch::start_new();
        for  _i in 1..1000
        {
            get_drinking_message_via_if(_age);
        }
        total += sw.elapsed_ms();
    }
    
    let first_average = total as f32 / runs as f32;
    println!("get_drinking_message_via_if average of {}ms", first_average );
    
    total = 0;    
    for  _i in 1..runs
    {
        _age = rng.gen_range(17,35);
        sw = Stopwatch::start_new();
        for  _i in 1..1000
        {
            get_drinking_message_via_logical(_age);
        }
        total += sw.elapsed_ms();
    }

    let second_average = total as f32 / runs as f32;
    println!("get_drinking_message_via_logical took {}ms", second_average);
    
}

fn get_drinking_message_via_if(_age: i32) -> &'static str {
    if _age < 18 {
        return  &USER_MESSAGES[0];
    }
    else if _age < 21 {
        return  &USER_MESSAGES[1];
    }
    else if _age < 25 {
        return  &USER_MESSAGES[2];
    }
    return  &USER_MESSAGES[3];
}
fn get_drinking_message_via_logical(_age: i32) -> &'static str{
    let value: usize =  (_age < 18) as usize + (_age < 21) as usize  + (_age < 25) as usize;
    return &USER_MESSAGES[value];
}

/* Rust wont let me cast int to bool
fn get_drinking_message_via_cast(_age: i32) -> String{
    let value: usize =  ((_age - 18) as bool) as usize  + ((_age - 21) as bool) as usize  + ((_age - 25) as bool) as usize;
    return USER_MESSAGES[value];
}*/

