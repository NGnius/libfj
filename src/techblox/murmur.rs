use fasthash::murmur3::hash32_with_seed;
use std::sync::mpsc::{channel, Sender};
use std::thread;

const HASH_SEED: u32 = 4919;

const ASCII_LETTERS: &[u8] = &[65,66,67,68,69,70,71,72,73,74,75,76,77,78,79,80,81,82,83,84,85,86,87,88,89,90, // A..Z
97,98,99,100,101,102,103,104,105,106,107,108,109,110,111,112,113,114,115,116,117,118,119,120,121,122]; // a..z

const ASCII_NUMBERS: &[u8] = &[48, 49, 50, 51, 52, 53, 54, 55, 56, 57]; // 0..9

const HASHNAME_ENDING: &[u8] = &[69, 110, 116, 105, 116, 121, // Entity
68, 101, 115, 99, 114, 105, 112, 116, 111, 114, // Descriptor
86, 42]; // EntityDescriptorV0

const MAX_LENGTH: usize = 10;

pub fn hashname(name: &str) -> u32 {
    hash32_with_seed(name, HASH_SEED)
}

pub fn brute_force(hash: u32) -> String {
    let (tx, rx) = channel::<String>();
    let mut start = Vec::<u8>::new();
    thread::spawn(move || brute_force_letter(hash, &mut start, &tx, 1));
    //println!("All brute force possibilities explored");
    if let Ok(res) = rx.recv_timeout(std::time::Duration::from_secs(30)) {
        return res;
    } else {
        return "".to_string();
    }
}

fn brute_force_letter(hash: u32, start: &mut Vec<u8>, tx: &Sender<String>, threadity: usize) {
    if start.len() > 0 {
        brute_force_endings(hash, start, tx);
    }
    if start.len() >= MAX_LENGTH { // do not continue extending forever
        //handles.pop().unwrap().join().unwrap();
        return;
    }
    let mut handles = Vec::<thread::JoinHandle::<_>>::new();
    start.push(65); // add letter
    let last_elem = start.len()-1;
    for letter in ASCII_LETTERS {
        start[last_elem] = *letter;
        if threadity > 0 {
            //thread::sleep(std::time::Duration::from_millis(50));
            let mut new_start = start.clone();
            let new_tx = tx.clone();
            handles.push(thread::spawn(move || brute_force_letter(hash, &mut new_start, &new_tx, threadity-1)));
        } else {
            brute_force_letter(hash, start, tx, threadity);
        }
    }
    for handle in handles {
        handle.join().unwrap()
    }
    start.truncate(last_elem);
}

fn brute_force_endings(hash: u32, start: &mut Vec<u8>, tx: &Sender<String>) {
    start.extend(HASHNAME_ENDING); // add ending
    let last_elem = start.len()-1;
    for num in ASCII_NUMBERS {
        start[last_elem] = *num;
        if hash32_with_seed(&start, HASH_SEED) == hash {
            let result = String::from_utf8(start.clone()).unwrap();
            println!("Found match `{}`", result);
            tx.send(result).unwrap();
        }
    }
    start.truncate(start.len()-HASHNAME_ENDING.len()); // remove ending
}
