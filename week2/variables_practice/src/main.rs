fn main() {
    //1.
    let moje_godine:u32 = 32;

   //2.
   let mut omiljeni_broj:u32 = 11;
   omiljeni_broj = omiljeni_broj+2;
   
   //3.
   let text:&str = "Ovo je neki tekst";
   let text:usize = text.len();         //metoda .len() uvek vraca usize


    println!("Ja imam {} godine.", moje_godine);
    println!("Moj novi omiljeni broj je {}.", omiljeni_broj);
    println!("Duzina unetog teksta je: {}",text);
}
