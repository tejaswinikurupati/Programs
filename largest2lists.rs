fn main () {
    let number_list=vec![34,50,25,65,100];
    let mut largest=&number_list[0];
    for number in &number_list{
        if number>largest{
            largest=number;

        }
    }
    println!("the largest number is {}",largest);
    let number_list=vec![104,34,6000,89,54,2,43,8];
    let mut largest=&number_list[0];
    for number in &number_list{
        if number>largest{
            largest=number;
        }
    }
    println!("the largest number is {}",largest);
}
