extern crate argon2min;

pub fn main() {
    let (password, salt) = ("argon2i!", "delicious salt");
    println!("argon2i_simple(\"{}\", \"{}\"):", password, salt);
    for byte in argon2min::argon2i_simple(&password, &salt).iter() {
        print!("{:02x}", byte);
    }
    println!();
}
