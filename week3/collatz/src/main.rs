fn main() {
    
   let mut counter = 0;
   let mut starting_number = 14;
   print!("Collatz sekvenca: ");
   print!("{} ",starting_number);
   
   loop{
     
   if starting_number%2==0{
      starting_number = starting_number/2;
      counter += 1;
      print!("{} ", starting_number);
   }else {
       starting_number = starting_number*3+1;
       counter += 1;
       print!("{} ",starting_number);

   }

   if starting_number == 1{
      counter += 1;
      break;
   }

   }
   print!("\n");     //novi red
   println!("Duzina sekvence (broj brojeva nakon pocetnog do broja 1) je: {} ", counter-1); //oduzimam pocetni broj
}

