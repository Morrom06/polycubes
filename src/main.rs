mod block_arrangement;
mod mapper;
mod point;
mod block_hash;
mod orientation;
mod poly_tree;

use std::{env, io};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufReader, BufWriter, Error, ErrorKind};
use std::ops::RangeInclusive;
use crate::block_arrangement::block_variation::VariationGenerator;
use crate::block_arrangement::BlockArrangement;
use crate::block_hash::BlockHash;

/// This program calculates out how many unique arrangements can be made for n cubes attached to one another
/// at the faces.
fn main() {
    let mut args = env::args();
    args.next();
    let n: u8 = args.next().map(|s| s.parse())
        .expect("Expected at least one numeric arguments")
        .expect("The argument has to be a valid number");
    let num = calc_num_of_unique_arrangements(n);
    println!("The number of arrangements is {num}");
}

fn calc_num_of_unique_arrangements(num_blocks: u8) -> usize {
    let next_highest_cache = load_next_highest_available_cache(num_blocks).ok();
    // Check if already generated
    let (starting_cache, cache_block_num) = if let Some((cache, num)) = next_highest_cache {
        if num == num_blocks {
            println!("Found precomputed cache for {num}.");
            return cache.len();
        } else {
            println!("Found precomputed cache for {num}.");
            (cache, num)
        }
    } else {
        println!("Found no cached data, starting from scratch");
        let mut cache = Cache::new();
        let ba = BlockArrangement::new();
        cache.insert(BlockHash::from(&ba), ba);
        (cache, 1)
    };
    let mut cache = starting_cache;
    for generating_size in RangeInclusive::new(cache_block_num + 1, num_blocks) {
        let next_larger = generate_increased_variations_from_cache(&cache);
        if let Err(e) = save_computed_values(&next_larger) {
            eprintln!("Unable to save cache of size {generating_size} because: {e}");
        } else {
            println!("Saved computed values for arrangements of size: {generating_size}");
            println!("Found {} arrangements.", next_larger.len());
        }
        cache = next_larger;
    }
    cache.len()
}

fn generate_increased_variations_from_cache(cache: &Cache) -> Cache {
    cache.values()
        .flat_map(|v| VariationGenerator::new(v.clone()))
        .map(|v| (BlockHash::from(&v), v))
        .collect()
}

fn file_name_for_n_block_cache(num_blocks: u8) -> String {
    format!("{num_blocks}-blocks-cache.cac")
}

/// Loads if the next highest available cache and returns it and the block number
/// whe found.
fn load_next_highest_available_cache(num_blocks: u8) -> io::Result<(Cache, u8)> {
    for cache_num in (1..num_blocks).rev() {
        if let Ok(cache) = load_precomputed_values(cache_num) {
            return Ok((cache, cache_num));
        }
    }
    Err(Error::new(ErrorKind::NotFound, "No caches found"))
}

type Cache = BTreeMap<BlockHash, BlockArrangement>;

fn load_precomputed_values(num_blocks: u8) -> io::Result<Cache> {
    let file = File::open(file_name_for_n_block_cache(num_blocks))?;
    let mut buff_read = BufReader::new(file);
    let config = bincode::config::standard();
    bincode::serde::decode_from_reader(&mut buff_read, config)
        .map_err(|e| Error::new(ErrorKind::InvalidData, e))
}

fn save_computed_values(cache: &Cache) -> io::Result<()> {
    let Some(num_blocks) = cache.values().next().map(|arr| arr.num_blocks()) else {
        return Err(Error::new(ErrorKind::InvalidData, "The cahce does not contain any values"));
    };
    let file_path = file_name_for_n_block_cache(num_blocks);
    if let Err(e) = std::fs::remove_file(&file_path) {
        match e.kind() {
            ErrorKind::NotFound => {}
            _ => {
                return Err(e);
            }
        }
    }
    let file = File::create(&file_path)?;
    let mut writer = BufWriter::new(file);
    let config = bincode::config::standard();
    bincode::serde::encode_into_std_write(cache, &mut writer, config)
        .map_err(|e| Error::new(ErrorKind::InvalidData, e))
        .map(|_len| ())
}