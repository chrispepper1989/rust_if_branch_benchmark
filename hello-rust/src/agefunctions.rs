//string messages
const USER_MESSAGES: &'static [&'static str] = &["Sorry no beer for you", "Not in the USA", "I will need to check you", "sure you can drink!"];

pub fn get_drinking_message_via_if(_age: i32) -> &'static str {
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

pub fn get_drinking_message_via_logical(_age: i32) -> &'static str{
    let value: usize =  (_age < 18) as usize + (_age < 21) as usize  + (_age < 25) as usize;
    return &USER_MESSAGES[value];
}

/* Rust wont let me cast int to bool
fn get_drinking_message_via_cast(_age: i32) -> String{
    let value: usize =  ((_age - 18) as bool) as usize  + ((_age - 21) as bool) as usize  + ((_age - 25) as bool) as usize;
    return USER_MESSAGES[value];
}*/