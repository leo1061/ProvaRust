use sharks::{ Sharks, Share };
use rand;


fn main() {

    // Set a minimum threshold of 10 shares
    let threshold = 10;
    let sharks = Sharks(threshold);
    println!("Created Sharks instance with threshold {}", threshold);

    // Obtain an iterator over the shares for secret [1, 2, 3, 4]
    let secret: [u8; 16] = rand::random();
    let dealer = sharks.dealer(&secret);
    println!("Created dealer for secret {:?}", secret);

    // Get 10 shares
    let shares: Vec<Share> = dealer.take(threshold as usize).collect();
    // Recover the original secret!
    let recovered_secret = sharks.recover(shares.as_slice()).unwrap();
    println!("Recovered secret: {:?}", recovered_secret);

    assert_eq!(recovered_secret, secret);
    println!("Secret matches original!");

}