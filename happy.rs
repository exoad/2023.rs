#![allow(uncommon_codepoints)]
fn main(){print!("\n\n\n\n\n");let mut ᚠ=[0x48u8,0x61,0x70,0x70,0x79,0x20,0x4E,0x65,0x77,0x20,0x59,0x65,0x61,0x72,0x21];(0..100).for_each(|_|(0..ᚠ.len()-5).for_each(|ᛒ|{let ᚷ=((ᚠ[ᛒ]as f64*13.579).sin()as usize)%ᚠ.len();ᚠ.swap(ᛒ,ᚷ);}));ᚠ.iter().for_each(|&ᛒ|print!("{}",char::from_u32(ᛒ as u32).unwrap()));print!("\n\n\n\n\n");}
// ~ exoad 2023 out
// ! https://github.com/exoad/2023.rs

