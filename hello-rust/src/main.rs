extern crate stopwatch;
use stopwatch::{Stopwatch};
use rand::Rng;
mod agefunctions;
use crate::agefunctions::get_drinking_message_via_if;
use crate::agefunctions::get_drinking_message_via_logical;
use crate::agefunctions::get_drinking_message_via_logical_index;

fn main() {

    println!("Demo that the two approaches are equivalent");
    println!("-------------------------------------");
    for  age in 15..30
    {
        let message1 = get_drinking_message_via_if(age);
        println!("if   Version: you are {}, therefore {}",age, message1);

        let value = get_drinking_message_via_logical_index(age);
        let message2 = get_drinking_message_via_logical(age);
        println!("Logic Version: you are {}, therefore {} because index was {}",age, message2, value);
    }

    println!("\n\nBasic benchmark");
    println!("-------------------------------------");
    custom_benchmark();
}
fn custom_benchmark()
{
    let mut rng = rand::thread_rng();
    let mut _age = rng.gen_range(17,35);    
    let mut sw;    
    let mut total = 0;
    let runs = 1000000;

    for  _i in 1..runs
    {      
        sw = Stopwatch::start_new();
        for  _j in 1..100
        { 
            for  age in 15..35
            {
                get_drinking_message_via_if(age);
            }
        }
        total += sw.elapsed_ms();
    }
    
    let first_average = total as f32 / runs as f32;
    println!("get_drinking_message_via_if average of {}ms", first_average );
    
    total = 0;    
    for  _i in 1..runs
    {    
        sw = Stopwatch::start_new();
        for  _j in 1..100
        { 
            for  age in 15..35
            {
                get_drinking_message_via_logical(age);
            }
        }
        total += sw.elapsed_ms();
    }

    let second_average = total as f32 / runs as f32;
    println!("get_drinking_message_via_logical took {}ms", second_average);
    
}


