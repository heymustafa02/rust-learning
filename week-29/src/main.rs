trait Shape {
    fn area(&self) -> u32; 
}

struct Rect {
    width: u32,
    height: u32,

}
impl Shape for Rect{
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

struct Circle {
    radius: u32,
}
impl Shape for Circle{
    fn area(&self) -> u32 {
        self.radius * self.radius
    }
}

fn main() {
   let r: Rect = Rect {
        width: 10,
        height: 20,
   };
   let c: Circle = Circle {
        radius: 15,
   };
    get_area(r);
    get_area(c);
    println!("Area: {}", get_area(r));
}
fn get_area(s:impl Shape) -> u32 {
    return s.area()
}