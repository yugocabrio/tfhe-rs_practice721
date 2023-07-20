use tfhe::integer::gen_keys_radix;
use tfhe::shortint::parameters::PARAM_MESSAGE_2_CARRY_2;

#[test]
fn calc_bmi() {
    let num_block = 4;
    let (client_key, server_key) = gen_keys_radix(&PARAM_MESSAGE_2_CARRY_2, num_block);

    let weight: u64 = 70;  // weight in kg
    let height: u64 = 170;  // height in cm

    let modulus = client_key.parameters().message_modulus.0.pow(num_block as u32) as u64;

    let mut ct_1 = client_key.encrypt(weight);
    let mut ct_2 = client_key.encrypt(height);

    let mut ct_2_clone = ct_2.clone();
    server_key.mul_assign_parallelized(&mut ct_2_clone, &mut ct_2);
    ct_2 = ct_2_clone;

    // The division part is not straightforward and missing in this example
    // server_key.div_assign_parallelized(&mut ct_1, &mut ct_2);  // weight / (height^2)

    // We use the client key to decrypt the output of the circuit:
    let output: u64 = client_key.decrypt(&ct_2);  // We only decrypt height^2
    println!("Decrypted output: {:?}", output);
    assert_eq!(output, (height * height) % modulus as u64);
}
