fn main(){

let studenti: [(&str, u8, f32); 3] = [("Milan", 22, 7.48), ("Milica", 23, 8.52), ("Marija", 21, 9.00)];

//tabela
println!("Podaci o studentima:");
println!("{:<9} {:<12} {:<12}", "Ime:" , "Godine:" , "Prosek:");
println!("{:<9} {:<12} {:<12}", studenti[0].0, studenti[1].0, studenti[2].0);
println!("{:<9} {:<12} {:<12}", studenti[0].1, studenti[1].1, studenti[2].1);
println!("{:<9} {:<12} {:<12}", studenti[0].2, studenti[1].2, studenti[2].2);

//prosecna ocena
let mut suma:f32 = 0.0;
 
for i in 0..studenti.len(){
    suma = suma + studenti[i].2;
}

let prosecna_ocena:f32 = suma/studenti.len() as f32;

println!("Prosecna ocena navedena tri studenta je: {}", prosecna_ocena);

}
