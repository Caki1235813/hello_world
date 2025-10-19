fn is_even(n: i32) -> bool {
    
    if n%2 == 0 {
        true
    } else {
        false
    }
}

fn main() {
    
    println!("Da li su brojevi izmedju 1 i 10 parni ili neparni?");
    
    for i in 1..11{

        if is_even(i){
            println!("Broj {} je paran.", i);
        }
        else{
            println!("Broj {} je neparan.", i);
        }
    }


}
