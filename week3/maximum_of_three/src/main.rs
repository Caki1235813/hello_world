fn max_of_three(a: i32, b: i32, c: i32) -> i32 {
    
    if a > b {

        if a > c {
            return a
        } else{
            return c
            }
    } else {

        if b > c {
            return b
            
        } else {
            return c
            }
        }
    }
fn main () {
        
        let max:i32 = max_of_three(-15,-23,100);
        println!(" Maximum unetih brojeva je {}." , max );
    

}
