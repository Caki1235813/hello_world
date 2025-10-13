fn main(){

// Integer:

//i8 (min = -128, max = 127):
let temp_zima:i8 = 0b11 - 0b111;
let temp_leto:i8 = 31;
println!("Temperatura zimi je {}, a temperatura leti je {}",temp_zima, temp_leto); 
//u8 (min = 0, max = 255):
let _starosno_doba:u8 = 58;
//i16 (min = -32768, max = 32767)
let broj_dana:i16 = 0xff+100;
let dozvoljeni_minus:i16 = -20_000;
//u16 (min = 0, max = 65535):
let cena:u16 = 40_000;
println!("Na {} i cenu od {} dozvoljeni minus je {}", broj_dana, cena, dozvoljeni_minus);
//i32 (min = - -2,147,483,648, max = 2,147,483,647):
let _dug:i32 = -2_100_000_000;
let br_vozila:i32 = 1_140_431_200;
//u32(min = 0, max = 4,294,967,295):
let br_stanovnika:u32 = 3_000_000_000;
println!("U proseku na oko {} stanovnika bilo bi oko {}", br_stanovnika, br_vozila);
//i64 (min = -2^63. max = 2^63-1):
let negativan_broj:i64= -8_000_000_000_000_000_001;
let pozitivan_broj:i64 = 8_000_000_500_000_001_001;
//u64(min = 0, max = 2^64-1):
let velicina:u64 = 18_446_744_073_709_551_610;
println!("Zbir {}+{} je manji od {}", negativan_broj, pozitivan_broj, velicina);
// isize:
let unos_korisnika:isize = "-21".parse().unwrap();
println!("{}", unos_korisnika);   //provera
//usize:
let ime_korisnika:usize = "Rusty".len();
println!("Duzina imena korisnika je {}", ime_korisnika);  //provera


//Floating points:

//f64
let razlika = 3.211_f64 - 5.4123_f64;
let zbir:f64 = 18.1234566666+0.001;
println!("Zbir je {}, a razlika je {}", zbir, razlika);
//f32:
let povrsina:f32 = 3.45*4.35;
let dubina :f32 = - 15.23;
println!("Povrsina je {}, a dubina je {}",povrsina,dubina );

// Boolean:
let type1 = true;
let type2 = false;

let and:bool = type1 && type2;
let or:bool = type1 || type2;

println!("Rezulatat AND operacije nad TRUE i FALSE je {}, a rezulatat OR operacije nad TRUE i FALSE je {}",and,or);

if !type2 {
    println!("Dobili smo true zbog negacije");
} else {
   println!("Ovo ne bi trebalo da vidimo u ovom primeru");
}

//Char:
let slovo1:char = 'Č';
let slovo2:char = 'a';
let slovo3:char = 'k';
let slovo4:char = 'i';
let space:char = ' ';
let little:char = '🤏';
let elephant:char = '🐘';

println!("{}{}{}{}{}{}{}", slovo1, slovo2, slovo3,slovo4, space, little, elephant);








}
