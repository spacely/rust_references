fn main() {
    let s = "Hello Saheed";
    println!("Hello, world!");
    {
        let s = "Big man";
        println!("{}",s);
    }
    println!("{}",s);
    let mut s = String::from("This is Ifedayo");
    s.push_str("And Ebun");
    println!("{}",s);
    let x = 7;
    let y = x;
    println!("{}",y);
    let  s1 = String::from("A test string initially");
    let s2 = s1;
    println!("{}",s2);

    let mut a_string = String::from("A test string");
    change_string(&mut a_string);
    println!("{}",a_string);
    
}

fn change_string(some_string: &mut String){
    some_string.push_str("Amazing")

}
