struct point<T> {
    x:T,
    y:T,
}
impl<T> point<T> {
    fn x(&self) ->&T {
        &self.x
    }
}
fn main() {
    let p =point {x:5,y:10};
    println!("p.x={}",p.x());
}
