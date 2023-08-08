mod block_arrangement;
mod mapper;
mod point;
mod block_hash;
mod orientation;

use std::collections::BTreeMap;
use std::{env, io, mem};
use std::fs::File;
use std::io::{BufReader, BufWriter, Error, ErrorKind, Write};
use crate::block_arrangement::block_variation::VariationGenerator;
use crate::block_arrangement::BlockArrangement;
use crate::block_hash::BlockHash;

/// This program calculates out how many unique arangements can be made for n cubes attached to one another
/// at the faces.
fn main() {
    let mut args = env::args();
    let _program_path = args.next();
    // let n: usize = args.next()
    //     .map(|s| {
    //         println!("{s}");
    //         s.parse()
    //     })
    //     .expect("Expected at least one numeric arguments")
    //     .expect("The argument has to be a valid number");
    // let num_unique_shapes: usize = generate(n).last().unwrap().len();
    // println!("The number of unique arrangements of {n} blocks is {num_unique_shapes}");
}

fn generate(n: usize) -> Vec<BTreeMap<BlockHash, BlockArrangement>> {
    let mut initial_map = BTreeMap::new();
    let ba = BlockArrangement::new();
    initial_map.insert(BlockHash::from(&ba), ba);
    let mut block_sets: Vec<BTreeMap<BlockHash, BlockArrangement>> = vec![
        initial_map,
    ];
    let mut starting_block_size = 1;
    if let Some((cache, block_num)) = load_next_lowest_cache(n) {
        block_sets.push(cache);
        starting_block_size = block_num;
    }

    for source_block_size in starting_block_size..n {
        let generated_block_size = source_block_size + 1;
        print!("Generating shapes with {generated_block_size} blocks...");
        io::stdout().flush().expect("Unable to flush stout");
        let new_blocks = generate_variants_from(block_sets.last().unwrap().values());
        println!("Done");
        print!("Saving cache data arrangements with {generated_block_size} blocks...");
        io::stdout().flush().expect("Unable to flush stout");
        // if source_block_size == 2 {
        //     dbg!(&new_blocks.iter().map(|b|
        //         b.center_mass_iter().collect::<Vec<_>>()
        //     ).collect::<Vec<_>>());
        // }
        match save_cache(&new_blocks, generated_block_size) {
            Ok(_) => {
                println!("Saved cache with {} items.", new_blocks.len())
            }
            Err(e) => {
                eprintln!("Failed to save cache data: {e}")
            }
        }
        block_sets.push(new_blocks);
    }
    block_sets
}

/// Attempts to load the cache with the largest block size lower that block_num
/// that can be found.
fn load_next_lowest_cache(block_num: usize) -> Option<(BTreeMap<BlockHash, BlockArrangement>, usize)> {
    for i in (2..block_num).rev() {
        println!("Attempting to load cache data for {i} blocks...");
        let res = load_cache(i);
        match res {
            Err(e) => {
                eprintln!("Failed load cache: {e}");
            }
            Ok(cache) => {
                println!("Loaded cache with {} items.", cache.len());
                return Some((cache, i));
            }
        }
    };
    None
}

fn load_cache(block_count: usize) -> Result<BTreeMap<BlockHash, BlockArrangement>, Error> {
    let file_name = gen_cache_file_name(block_count);
    let cache_file = File::open(file_name)?;
    let mut buff_reader = BufReader::new(cache_file);

    let config = bincode::config::standard();
    bincode::serde::decode_from_std_read(&mut buff_reader, config)
        .map_err(|e| Error::new(ErrorKind::InvalidData, e))
}

fn save_cache(set: &BTreeMap<BlockHash, BlockArrangement>, block_count: usize) -> Result<(), Error> {
    let file_name = gen_cache_file_name(block_count);
    if let Err(err) = std::fs::remove_file(&file_name) {
        match err.kind() {
            ErrorKind::NotFound => {}
            _ => {return Err(err)}
        }
    }
    let cache_file = File::create(&file_name)?;
    let mut writer = BufWriter::new(cache_file);

    let config = bincode::config::standard();
    bincode::serde::encode_into_std_write(set, &mut writer, config)
        .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
    Ok(())
}

fn gen_cache_file_name(block_count: usize) -> String {
    format!("./shape_cache_{block_count}.cac")
}

/// Generates variants of blocks from the given iterator and returns a set of those blocks.
fn generate_variants_from<'a>(iter: impl Iterator<Item = &'a BlockArrangement>) -> BTreeMap<BlockHash, BlockArrangement> {
    iter.flat_map(VariationGenerator::new)
        .map(|ba| (BlockHash::from(&ba), ba))
        .collect()
}