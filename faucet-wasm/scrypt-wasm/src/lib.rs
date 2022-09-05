extern crate hex;
extern crate scrypt;
extern crate wasm_bindgen;

use std::iter::repeat;
use std::mem::transmute;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn scrypt(password: &str, salt: &str, n: u32, r: u32, p: u32, dklen: usize) -> String {
    let err_str = String::from("input Error");
    let log_n = (32 - n.leading_zeros() - 1) as u8;
    if log_n as u32 >= r * 16 {
        return String::from("Invalid r");
    }
    if p as u64 > ((u32::max_value() as u64 - 1) * 32) / (128 * (r as u64)) {
        return String::from("Invalid p");
    }
    let mut result: Vec<u8> = repeat(0).take(dklen).collect();
    let params = match scrypt::ScryptParams::new(log_n, r, p) {
        Ok(params) => params,
        Err(_err) => return err_str,
    };
    let pass_ = match hex::decode(password) {
        Ok(p) => p,
        Err(_err) => return err_str,
    };
    let salt_ = match hex::decode(salt) {
        Ok(p) => p,
        Err(_err) => return err_str,
    };
    scrypt::scrypt(&pass_, &salt_, &params, &mut result).expect("Error executing scrypt");
    hex::encode(result)
}

#[wasm_bindgen]
pub fn mine(start: u64, end: u64, difficulty: u32, salt: &str, n: u32, r: u32, p: u32, dklen: usize) -> String {
    let err_str = String::from("input Error");
    let log_n = (32 - n.leading_zeros() - 1) as u8;
    if log_n as u32 >= r * 16 {
        return String::from("Invalid r");
    }
    if p as u64 > ((u32::max_value() as u64 - 1) * 32) / (128 * (r as u64)) {
        return String::from("Invalid p");
    }
    let mut result: Vec<u8> = repeat(0).take(dklen).collect();
    let params = match scrypt::ScryptParams::new(log_n, r, p) {
        Ok(params) => params,
        Err(_err) => return err_str,
    };
    let salt_ = match hex::decode(salt) {
        Ok(p) => p,
        Err(_err) => return err_str,
    };

    let diff_bytes = difficulty / 8;
    let diff_mask = 1 << (8 - (difficulty - (diff_bytes * 8)));

    for index in start..end {
        let pass_: [u8; 8] = unsafe { transmute(index.to_be()) };
        scrypt::scrypt(&pass_, &salt_, &params, &mut result).expect("Error executing scrypt");

        let mut valid = true;
        for byteidx in 0..diff_bytes {
            if result[byteidx as usize] > 0 {
                valid = false;
                break;
            }
        }
        if valid && (diff_mask == 0 || result[diff_bytes as usize] < diff_mask) {
            return format!("{}|{}", index, hex::encode(result));
        }
    }
    String::from("0")
}
