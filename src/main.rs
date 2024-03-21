use tfhe::{set_server_key, ClientKey, CompressedServerKey, ConfigBuilder, FheUint, FheUint8};
use tfhe::prelude::*;
use tfhe::shortint::parameters::PARAM_MULTI_BIT_MESSAGE_2_CARRY_2_GROUP_3_KS_PBS;
// time
use std::time::Instant;

fn main() {

    let config = ConfigBuilder::with_custom_parameters(PARAM_MULTI_BIT_MESSAGE_2_CARRY_2_GROUP_3_KS_PBS, None).build();

    let client_key= ClientKey::generate(config);
    let compressed_server_key = CompressedServerKey::new(&client_key);

    let gpu_key = compressed_server_key.decompress_to_gpu();

    let clear_a = 27u8;
    let clear_b = 128u8;

    let a = FheUint8::encrypt(clear_a, &client_key);
    let b = FheUint8::encrypt(clear_b, &client_key);


    //Server-side
    set_server_key(gpu_key);
    //let result = a + b;

    // time
    let now = Instant::now();
     let greater = a.gt(&b);

    let elapsed = now.elapsed();
    println!("Elapsed: {:?}", elapsed);

    //Client-side
    // decrypt min_value
    let greater_value = greater.decrypt(&client_key);
    println!("greater: {:?}", greater_value);

}