extern crate stopwatch;
use stopwatch::{Stopwatch};
use rand::Rng;
mod agefunctions;
use crate::agefunctions::get_drinking_message_via_if;
use crate::agefunctions::get_drinking_message_via_logical;

fn main() {
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


