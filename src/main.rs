fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        println!("counter: {}", counter);
        if counter == 10 {
            break counter*2;
        }
    };
    // println!("Result: {}", result);
    // sub();
    string_heap();
    str_clone();
    out_scope();

    let s = String::from("hello");
    take_ownership(s);
    // println!("c {}",c); => returns err since s is out of scope
    let c = 5;
    makes_copy(c);
    println!("c {}",c);

    let s1 = gives_ownership(); // gives_ownership moves its return value to s1
    let s2 = String::from("yours");// s comes into scope
    let s3 = takes_and_gives_back(s2);// s2 is moved into, take_and_gives_back movesits return vslue into s3

    let s1_calc = String::from("ola");
    let (s2_calc, len) = calculate_length(&s1_calc);
    println!("len of s1_calc: {} is {}", s2_calc, len);
    let mut  s_change = String::from("hello");
    let ret = change(&mut s_change);
    println!("return value {:?}", ret)
}// Here, c goes out of scope, then s. But because s's value was moved, nothing
// s3 goes out of scope and is dropped . s moved so nothing happens s1 goes out of sccope so nothing happens
  // special happens.
fn change(some_string:&mut String) -> &mut String{
    some_string.push_str(", world");
    some_string
}
fn calculate_length(s:&String) -> (String, usize){// & symbol is used as a refernce => does not take ownership
    let length = s.len();
    return (s.to_string(), length);  
}
// Transfer of ownership
fn gives_ownership() -> String{// fn will move its return value into the function that calls it
    let some_string = String::from("yours");// some string comes into scope
    some_string// some_string is returned and moves into the calling function
}
// takes in a string and returns one 
fn takes_and_gives_back(a_string: String) -> String {// a_string comes into scope
    a_string// a_string is returned and moves out to the calling function
}
  
fn take_ownership(some_str:String){ // some_str comes into scope
    println!("{}", some_str)// here some_str goes out of scope and drop is called THe backing memory is freed
}
fn makes_copy(some_int:i32){// some_int comes into scope
    println!("{}", some_int)// here some_int goes out of scope nothing special happens 
}
fn string_heap(){
    let mut s = String::from("Hello");
    s.push_str(", world!");// push_str appends literal to a string, 
    println!("{s}");
}// function drop is called as a garbage collector
fn out_scope(){
    let s = String::from("Hello");
    let s2 = s;  // at this point s goes out of scope
    // println!("{}, world!", s); returns err
}
fn str_clone(){
    let s = String::from("Hello");
    let s2 = s.clone();
    println!("s1 = {}, s2 = {}", s, s2);
}
fn sub() {
    let mut counter = 0;
    let resul: i32= loop {
        counter += 1;
        // println!("counter: {}", counter);
        if counter == 5{
            break counter*2;
        }
    };
    // println!("Result: {}", resul);
    conditional();
    collection();
    collectionwoindex();
    fxwshortcondition();
}
fn conditional(){
    let mut number = 3;

    while number != 0{
        // println!("number: {}", number);
        number = number - 1;
    }
    //  println!("LIFTOFF!!!");
}
fn collection() {
    let a  = [1,2,3,4,5];
    let mut index = 0;
    while index <5{
        // println!("index: {}", index);
        // println!("value: {}", a[index]);
        index = index + 1;
    }
}
fn collectionwoindex() {
    let a  = [1,2,3,4,5];
    for element in a{
        // println!("the element: {}", element);
    }
}
fn fxwshortcondition() {
    for number in (1..4) {
        // println!("number: {}", number);
    }
    println!("LIFTOFF!!!");
}