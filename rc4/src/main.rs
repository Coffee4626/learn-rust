mod rc4;
fn main() {
    let secret1 = encrypt_data!(b"Hello RC4 from Rust!");

    println!("Encrypted first string bytes:");
    for b in secret1.data.iter() {
        print!("{:02X} ", b);
    }
    println!();

    let decrypted1 = secret1.decrypt();
    println!(
        "Decrypted first string: {}",
        String::from_utf8_lossy(&decrypted1)
    );
    println!();

    let secret2 = encrypt_data!(b"The quick brown fox jumps over the lazy dog.");

    println!("Encrypted second string bytes:");
    for b in secret2.data.iter() {
        print!("{:02X} ", b);
    }
    println!();

    let decrypted2 = secret2.decrypt();
    println!(
        "Decrypted second string: {}",
        String::from_utf8_lossy(&decrypted2)
    );
}
