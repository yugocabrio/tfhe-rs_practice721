use tfhe::integer::gen_keys_radix;
use tfhe::shortint::parameters::PARAM_MESSAGE_2_CARRY_2;

#[test]
fn calc_bmi() {
    let num_block = 4;
    let (client_key, server_key) = gen_keys_radix(&PARAM_MESSAGE_2_CARRY_2, num_block);

    let weight: u64 = 150;  // weight in lb
    let height: u64 = 70;  // height in in
    let scaleFactor: u64 = 10000;
    let bmiFactor: u64 = 703;

    let modulus = client_key.parameters().message_modulus.0.pow(num_block as u32) as u64;

    let mut ct_1 = client_key.encrypt(weight);
    let ct_2 = client_key.encrypt(height);
    let mut ct_2_squared = ct_2.clone();  // clone for square operation
    let mut ct_3 = client_key.encrypt(scaleFactor);
    let mut ct_4 = client_key.encrypt(bmiFactor);

    server_key.mul_assign_parallelized(&mut ct_2_squared, &ct_2);  // height^2

    server_key.mul_assign_parallelized(&mut ct_1, &mut ct_4);  // weight * 703
    server_key.mul_assign_parallelized(&mut ct_1, &mut ct_3);  // weight * 703 * scaleFactor

    // We use the client key to decrypt the output of the circuit:
    let output: u64 = client_key.decrypt(&ct_1);  // We only decrypt weight * 703 * scaleFactor
    println!("Decrypted output (weight * 703 * scaleFactor): {:?}", output);
    assert_eq!(output, (weight * bmiFactor * scaleFactor) % modulus as u64);
}
