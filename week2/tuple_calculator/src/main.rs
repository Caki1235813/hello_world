

fn main(){

let tri_broja: (u8, f32, u32) = (15, -32.51, 1_000);   //stores 3 numbers in a tuple

let (broj_1, broj_2, broj_3) = tri_broja;   //destructures the tuple

//suma

let suma: f32 = broj_1 as f32 + broj_2 as f32 + broj_3 as f32; // da bih ih sabrala svi moraju biti istog tipa

println!("Suma članova tuple-a iznosi {}.", suma);

//average

let srednja_vrednost:f32 = suma / 3.0; // 3 smo napisali kao 3.0  jer za operaciju deljenja tipovi moraju biti isti

println!("Srednja vrednost unetih članova tuple-a je {}.", srednja_vrednost);

// product (proizvod brojeva)

let proizvod: f32 = tri_broja.0 as f32 * tri_broja.1 as f32 * tri_broja.2 as f32; // ili let proizvod: f64 = broj_1 * broj_2 * broj_3;

println!("Proizvod unetih članova tuple-a iznosi {}.", proizvod);


}