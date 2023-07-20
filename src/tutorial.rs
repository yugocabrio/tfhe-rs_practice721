use tfhe::integer::gen_keys_radix;
use tfhe::shortint::parameters::PARAM_MESSAGE_2_CARRY_2;

#[test]
fn tutorial() {
    let num_block = 4;
    let (client_key, server_key) = gen_keys_radix(&PARAM_MESSAGE_2_CARRY_2, num_block);

    let msg1 = 12;
    let msg2 = 11;
    let msg3 = 9;
    let scalar = 3;

    // message_modulus^vec_length
    let modulus = client_key.parameters().message_modulus.0.pow(num_block as u32) as u64;

    // We use the client key to encrypt two messages:
    let mut ct_1 = client_key.encrypt(msg1);
    let mut ct_2 = client_key.encrypt(msg2);
    let mut ct_3 = client_key.encrypt(msg3);

    server_key.scalar_mul_assign_parallelized(&mut ct_1, scalar);

    server_key.sub_assign_parallelized(&mut ct_1, &mut ct_2);

    server_key.add_assign_parallelized(&mut ct_1, &mut ct_3);

    // We use the client key to decrypt the output of the circuit:
    let output: u64 = client_key.decrypt(&ct_1);
    println!("Decrypted output: {:?}", output);
    assert_eq!(output, ((msg1 * scalar as u64 - msg2) + msg3) % modulus as u64);
}