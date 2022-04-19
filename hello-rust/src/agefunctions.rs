//string messages
const USER_MESSAGES: &'static [&'static str] = &[
"Sure you can drink!",
"I will need to check your ID",
"Not in the USA",
"Sorry no beer for you"];


pub fn get_drinking_message_via_if(_age: i32) -> &'static str {
    if _age < 18 {
        return  &USER_MESSAGES[3];
    }
    else if _age < 21 {
        return  &USER_MESSAGES[2];
    }
    else if _age < 25 {
        return  &USER_MESSAGES[1];
    }
    return  &USER_MESSAGES[0];
}

pub fn get_drinking_message_via_logical(_age: i32) -> &'static str{
    let value: usize =  (_age < 18) as usize + (_age < 21) as usize  + (_age < 25) as usize;
    return &USER_MESSAGES[value];
}

pub fn get_drinking_message_via_logical_index(_age: i32) -> usize{
    let value: usize =  (_age < 18) as usize + (_age < 21) as usize  + (_age < 25) as usize;
    return value;
}
/* Rust wont let me cast int to bool
fn get_drinking_message_via_cast(_age: i32) -> String{
    let value: usize =  ((_age - 18) as bool) as usize  + ((_age - 21) as bool) as usize  + ((_age - 25) as bool) as usize;
    return USER_MESSAGES[value];
}*/