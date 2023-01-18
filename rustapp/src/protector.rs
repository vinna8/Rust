extern crate rand;

use rand::Rng;
use std::char;

pub fn get_session_key() -> String {
    /*generate 10 char random string*/
    let mut key = String::new();
    let mut rng = rand::thread_rng();

    for _i in 0..10 {
        let num = rng.gen_range(1, 10);
        let ch = char::from_digit(num, 10).unwrap();
        key.push(ch);
    }

    return key;
}

pub fn get_hash_str() -> String {
    /*calculate initial hash string*/
    let mut hash_str = String::new();
    let mut rng = rand::thread_rng();

    for _i in 0..5 {
        let num = rng.gen_range(1, 7);
        let ch = char::from_digit(num, 10).unwrap();
        hash_str.push(ch);
    }

    return hash_str;
}

pub fn next_session_key(hash_str: &str, session_key: &str) -> String {
    /*generate next session key*/
    /*verify hashcode*/
    if hash_str.is_empty() {
        return "Hash code is empty".to_string();
    }

    for ch in hash_str.chars() {
        if !ch.is_ascii_digit() {
            return "Hash code contains non-digit letter".to_string();
        }
    }

    let mut result = 0;

    for ch in hash_str.chars() {
        let l = ch.to_string();
        result += calc_hash(session_key.to_string(), l.parse::<u64>().unwrap())
            .parse::<u64>()
            .unwrap();
    }

    return result.to_string();
}

fn calc_hash(key: String, value: u64) -> String {
    /*calculate hash*/
    match value {
        1 => {
        let chp = "00".to_string() + &(key[0..5].parse::<u64>().unwrap() % 97).to_string();
        return chp[chp.len() - 2..chp.len()].to_string()
        }
        2 => {
            let reverse_key = key.chars().rev().collect::<String>();
            return reverse_key + &key.chars().nth(0).unwrap().to_string()
        }
        3 => {
            return key[key.len() - 5..key.len()].to_string() + &key[0..5].to_string()
        }
        4 => {
            let mut num = 0;
            for _i in 1..9 {
                num += key.chars().nth(_i).unwrap().to_digit(10).unwrap() as u64 + 41;
            }
            return num.to_string();
        }
        5 => {
            let mut ch: char;
            let mut num = 0;

            for _i in 0..key.len() {
                ch = ((key.chars().nth(_i).unwrap() as u8) ^ 43) as char;
                if !ch.is_ascii_digit() {
                    ch = (ch as u8) as char;
                }
                num += ch as u64;
            }
            return num.to_string();
        }
        _ => return (key.parse::<u64>().unwrap() + value).to_string(),
    }
}