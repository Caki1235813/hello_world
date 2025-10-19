fn fizzbuzz(n:i32) {
    
 for i in 1..=n {
   
  if i%3==0 && i%5==0{
     print!("FizzBuzz ")
  }else if i%3==0 {
      print!("Fizz ")
  }else if i%5==0 {
      print!("Buzz ")
  }else {
      print!("{} ",i)
  }


 };



}
fn main(){
    println!("Brojevi od 1 do 100 ispisani kroz FizzBuzz metod: {:?}", fizzbuzz(100));  //mora se pisati {:?} umesto {} jer fn vraca (), unit type nema Display trait
    
}
 