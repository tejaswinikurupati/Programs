use std::collections::HashMap;
fn main(){
    let mut stateCodes =HashMap::new();
    stateCodes.insert("GH","GUJARATH");
    stateCodes.insert("KL","KERALA");
    stateCodes.insert("MH","MAHARASTRA");
    println!("{:?}",stateCodes);
}
