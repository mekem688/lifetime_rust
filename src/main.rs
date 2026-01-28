use std::io;
fn action <'a> (s1:&'a str, s2: &'a str) -> &'a str {
    if s1.len() < s2.len() {
        s2

    }else {
    
        s1
    }
}
fn main() {
    println!("Hello, world!");
    println!("entre un mot :  ");
   let mut  mot1: String = String::new();
   io::stdin().read_line(&mut mot1).unwrap();
   let mot1: &str= mot1.trim();
   println!("entre le mot2: ");
   let mut mot2: String = String::new();
    io::stdin().read_line(&mut mot2).unwrap();
   let mot2: &str= mot2.trim();
   let teste: &str = action(&mot1, &mot2);

   println!("voicci le resultat du lifetime {} est le plus grand mot ",teste);

    


}
