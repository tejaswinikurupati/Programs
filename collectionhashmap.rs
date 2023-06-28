fn main(){
use std::collections::HashMap;
let mut scores=HashMap::new();
scores.insert(string::from("blue"),10);
scores.insert(string::from("yellow"),50);
for (key,value) in &scores {
    println!("{key}:{value}");
}
}
