mod utils;

use std::error::Error;
use std::fmt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasmtest!");
}


#[wasm_bindgen]
pub struct StoreResponse {
    pub measured_ph: f64,
    pub fermenter_top_up: f64,
}

#[derive(Debug)]
struct TooManyBottles;

impl Error for TooManyBottles {}

impl fmt::Display for TooManyBottles {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Up to 94 bottles are supported")
    }
}

impl Into<JsValue> for TooManyBottles {
    fn into(self) -> JsValue {
        JsValue::from_str("Up to 94 bottles are supported")
    }
}

#[wasm_bindgen]
pub fn store_bottle_ids(measured_ph: f64, fermenter_top_up: f64, bottle_ids: &[i32]) -> Result<StoreResponse, TooManyBottles> {
    log("writing bottle ids");
    log(&format!("original measured_ph: {}", measured_ph));
    log(&format!("original fermenter_top_up: {}", fermenter_top_up));
    log(&format!("bottle_ids to store: {:?}", bottle_ids));
    if bottle_ids.len() > 94 {
        return Err(TooManyBottles);
    }

    if bottle_ids.iter().any(|&id| id < 1 || id > 94) {
        return Err(TooManyBottles);
    }

    let one_to_fifty_two = bottle_ids.iter().filter(|&&id| id <= 52).map(|&id| id).collect::<Vec<i32>>();
    log(&format!("one_to_fifty_two: {:?}", one_to_fifty_two));
    let fifty_three_to_ninety_four = bottle_ids.iter().filter(|&&id| id > 52).map(|&id| id - 52).collect::<Vec<i32>>();
    log(&format!("fifty_three_to_ninety_four: {:?}", fifty_three_to_ninety_four));

    let measured_ph = store_array_64(&measured_ph, one_to_fifty_two, true);
    log(&format!("new measured_ph: {}", measured_ph));
    let fermenter_top_up = store_array_64(&fermenter_top_up, fifty_three_to_ninety_four, false);
    log(&format!("new fermenter_top_up: {}", fermenter_top_up));

    Ok(StoreResponse{
        measured_ph,
        fermenter_top_up,
    })
}

#[wasm_bindgen]
pub fn read_bottle_ids(measured_ph: f64, fermenter_top_up: f64) -> Box<[JsValue]> {
    log("reading bottle ids");
    log(&format!("original measured_ph: {}", measured_ph));
    log(&format!("original fermenter_top_up: {}", fermenter_top_up));
    let mut fifty_three_to_ninety_four = read_array_64(fermenter_top_up, false).iter().map(|&id| id + 52).collect::<Vec<i32>>();
    log(&format!("fifty_three_to_ninety_four: {:?}", fifty_three_to_ninety_four));
    let mut one_to_fifty_two = read_array_64(measured_ph, true);
    log(&format!("one_to_fifty_two: {:?}", one_to_fifty_two));
    one_to_fifty_two.append(&mut fifty_three_to_ninety_four);
    one_to_fifty_two.iter().map(|&id| JsValue::from(id)).collect::<Vec<JsValue>>().into_boxed_slice()
}

fn store_array_64(db: &f64, bottle_ids: Vec<i32>, steganographic: bool) -> f64 {
    let mut db = db.to_bits();

    // unset the bits we have to play with - this effectively zeroes the 'db'
    let mask_least_significant_bits = if steganographic {
        0b_11111111_11111111_11111100_00000000_00000000_00000000_00000000_00000000_u64
    } else {
        db = 105553116266498.08_f64.to_bits(); // need to make this a 'safe' value with at most 6 decimal digits, otherwise BrewFather will truncate it
        0b_11111111_11110000_00000000_00000000_00000000_00000000_00000000_00000000_u64
    };
    db &= mask_least_significant_bits;

    // set the bits that match the bottle ids
    let mut indices = vec![0; 64];
    for bottle_id in bottle_ids {
        indices[64 - bottle_id as usize] = 1;
    }
    let indices = indices.into_iter().fold(0, |acc: u64, digit:u64| (acc << 1) + digit);

    // xor the matching bits
    db |= indices;

    f64::from_bits(db)
}

fn read_array_64(db: f64, steganographic: bool) -> Vec<i32> {
    let db = db.to_bits();

    let mut indices = vec![0; 64];
    for i in 0..64 {
        indices[i] = (db >> i) & 1;
    }

    let size = if steganographic { 42 } else { 52 };

    let mut bottle_ids: Vec<i32> = vec![];
    for i in 0..size {
        if indices[i] == 1 {
            bottle_ids.push(i as i32 + 1);
        }
    }

    bottle_ids
}
