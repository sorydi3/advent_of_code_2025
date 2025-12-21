


type Board = Vec<Vec<char>>;
#[derive(Clone,Debug)]
struct Shape {
    pub id:u8,
    shape: Board
}

impl Default for Shape {
    fn default() -> Self { 
        Self {
            id: u8::default(),
            shape: vec![vec!['.';3];3]
        }
     }
}

impl Shape {

    fn new() {
        todo!()
    }
    fn rotate(&self) -> Self {todo!()}
    fn reflecte(&self) -> Self {todo!()}
    pub fn shapes(&self) -> Vec<Shape> {
        //only rotate
        // only reflecte
        // rotate and reflecte
        // reflecte and rotate
        vec![Shape::default();4]
    } 
}

#[derive(Clone,Debug)]
struct Region {
    region: Board
}

impl Region {
    fn new() {
        todo!()
    }
}




#[derive(Clone, Debug)]
struct Canva {
    counter:u8,
    regions: Vec<Region>,
    shapes:Vec<Shape>,
        
}

impl Default for Canva {
    fn default() -> Self {
        Self {
            regions: vec![],
            shapes:vec![],
            counter:0
        }   
    }
}




fn main() {
    println!("Hello, world!");
}
