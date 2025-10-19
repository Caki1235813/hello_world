fn sum_of_evens(n: u32) -> u32 {

let mut suma = 0;

for i in 1..=n {
    
    if i%2==0{
        suma = suma+i;
    }else {
        suma = suma+0;
    }
}

  suma
        
    
}

fn main() {
    println!("Sum of evens up to 20: {}", sum_of_evens(20));
}