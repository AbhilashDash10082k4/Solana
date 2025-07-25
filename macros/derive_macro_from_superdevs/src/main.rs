#[derive(Debug)] 
struct User {
    name: String,
    age: u32,
}
//implements Debug trait for User struct. It takes in Debug macro but implements Debug trait. It has its own implementation., cargo expand will expand the macro,//derive macros provide default implementation for a trait on a struct. This is Custom derive procedural macro 

//these impl statements are written by expanded derive procedural macros
// impl std::fmt::Display for User {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "name is {}, age is {}", self.name, self.age)
//     }
// }
// impl std::fmt::Debug for User {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "name is {}, age is {}", self.name, self.age)
//     }
// }
// #[get("/user/:{userid}")]  //attribute like macro
// fn create_user() {
//     sqlx::query!(INSERT INTO TABLE..) //function like macro
// }

//writing custom macro-
trait Serialize {
    fn serialize (&self) -> Vec<u8>;
}
trait Deserialize {
    fn deserialize(data: &[u8]) -> Result<Self,std::fmt::Error> where Self: Sized;
}
#[derive(Debug)]
struct Swap {
    qty_1: u32,
    qty_2: u32,
}
impl Serialize for Swap {
    fn serialize(&self) -> Vec<u8> {
        let mut v = vec![];
        v.extend_from_slice(&self.qty_1.to_be_bytes()); //to_be_bytes - [0,0,0,1], to_le_bytes -[1,0,0,0]
        v.extend_from_slice(&self.qty_2.to_be_bytes());
        return v;
    }
}
impl Deserialize for Swap {
    fn deserialize(data: &[u8]) -> Result<Self, std::fmt::Error> { //this does not has a self param, so this is a static/ associated fn
        if data.len() < 8 {
            return Err(std::fmt::Error);
        }
        let qty_1 = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);
        let qty_2 = u32::from_be_bytes([data[4], data[5], data[6], data[7]]);
        return Ok(Swap {qty_1, qty_2});
    }
}

fn main() {
    println!("Hello, world!");
    let user = User {name: "Abhi", age: 21};
    println!("{}", u); //display trait's fmt function
    println!("{:?}", u); //debug trait's fmt function,  println! is a declarative macro

    let s = Swap {qty_1: 1, qty_2: 2 };
    let v = s.serialize();
    let res = Swap::deserialize(&v).unwrap();
    println!("{:?}", v);
    println!("{:?}", res);
}
