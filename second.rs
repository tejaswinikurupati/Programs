struct Teju{
    gender:String,
    age:i32,
    weight:u8,
    height:u8,
}
fn main(){
    let mut x = Teju{
        gender:String::from("female"),
        age:23,
        weight:60,
        height:225,
    };
    println!("{}{}{}{}",x.gender,x.age,x.weight,x.height);
}
