pub fn is_armstrong_number(num: u32) -> bool {
    // unimplemented!("true if {} is an armstrong number", num)
    let s = &num.to_string();
    let len:u32 = num.to_string().len() as u32;

    let mut v:Vec<u32> = Vec::new();

    for i in s.chars(){
        // let char:&str = &i.to_string();
        v.push(i.to_digit(10).unwrap());
    };

    let armstrong = v.iter().map(|i| i.pow(len)).sum();
    num == armstrong
}
