use rand::Rng;
use serial_key::make_key; 

use tracing::info;

pub fn gen_api_id() -> String {
    let mut rng = rand::thread_rng();

    let rand1 = rng.gen_range(0..100);
    let rand2 = rng.gen_range(0..100);
    let rand3 = rng.gen_range(0..100);
    let rand4 = rng.gen_range(0..100);

    let seed = 0x3abc9099;

    let num_bytes = 4;
    let byte_shifts: Vec<(i16, i16, i16)> = vec![(rand1,rand2,rand3),(rand2,rand3,rand4),(rand3,rand4,rand1),(rand4,rand1,rand2)];
    let key = make_key(&seed, &num_bytes, &byte_shifts);
    info!("Api Id = {}", key);

    return key;
}

pub fn gen_api_secret () -> String {
    let mut rng = rand::thread_rng();
    let id = rng.gen::<u32>();
    info!("Api Secret: {}", id);

    return id.to_string();
}