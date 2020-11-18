#[path = "src/tables.rs"]
mod tables;

use std::cmp::Ordering;
use std::convert::TryFrom;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use block::{Block, LAST_INDEX};
use std::collections::{BTreeMap, BTreeSet};

const SHIFT: u32 = block::LAST_INDEX.count_ones();

type Row = ([u32; 2], [u32; 3], [u32; 3]);

fn main() {
    let output_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("case_mapping.rs");

    write_table(&output_path, &compile_table());
}

struct CompiledTable {
    blocks: Vec<(u32, Block)>,
    address_to_block_index: Vec<(u32, usize)>,
    last_code_point: u32,
    records: Vec<Row>,
}

fn compile_table() -> CompiledTable {
    let (mappings, codepoint_to_mapping_index) = compile_mappings();

    let mut blocks = Vec::new();
    let mut address_to_block_index = Vec::new();

    let start = *codepoint_to_mapping_index.keys().next().unwrap();
    let end = *codepoint_to_mapping_index.keys().last().unwrap();
    let last_code_point = end;

    // Extend end to the end of the last block to ensure the last block is written out
    let end_block_address = end & (!LAST_INDEX as u32);
    let end = end_block_address + block::SIZE as u32;

    let mut block = Block::new();
    for codepoint in start..=end {
        let mapping_index = codepoint_to_mapping_index
            .get(&codepoint)
            .copied()
            .unwrap_or(0);
        let block_address = (codepoint >> SHIFT).saturating_sub(1) << SHIFT;

        // This is the first codepoint in this block, write out the previous block
        if codepoint != 0 && (codepoint & u32::try_from(block::LAST_INDEX).unwrap()) == 0 {
            if let Some(index) = blocks.iter().position(|(_, candidate)| candidate == &block) {
                address_to_block_index.push((block_address, index));
            } else {
                // Add the block if it's new
                address_to_block_index.push((block_address, blocks.len()));
                blocks.push((block_address, block.clone()));
            }

            block.reset();
        }

        block[usize::try_from(codepoint).unwrap() & block::LAST_INDEX] = mapping_index;
    }

    CompiledTable {
        blocks,
        address_to_block_index,
        last_code_point,
        records: mappings,
    }
}

/// collects the lower, upper, titlecase mappings into one big table, noting the offset of each
/// code point in the table
fn compile_mappings() -> (Vec<Row>, BTreeMap<u32, u16>) {
    // Return the big table and a map from codepoint to offset within the table
    let mut mappings = vec![([0; 2], [0; 3], [0; 3])];
    let mut offsets = BTreeMap::new();
    // Add entry for empty, fallback row
    offsets.insert(0, 0);
    let mut codepoints: BTreeSet<_> = tables::LOWER.iter().map(|(cp, _)| *cp).collect();
    codepoints.extend(tables::UPPER.iter().map(|(cp, _)| cp));
    codepoints.extend(tables::TITLE.iter().map(|(cp, _)| cp));
    let start = *codepoints.iter().next().unwrap();
    let end = *codepoints.iter().last().unwrap();

    // for each code point lookup all the tables, create a row, add it to mappings
    for ch in start..=end {
        let lowercase = lookup(ch, tables::LOWER).map(|mapping| {
            let mut array = [0; 2];
            fill(mapping, &mut array);
            array
        });
        let uppercase = lookup(ch, tables::UPPER).map(|mapping| {
            let mut array = [0; 3];
            fill(mapping, &mut array);
            array
        });
        let titlecase = lookup(ch, tables::TITLE).map(|mapping| {
            let mut array = [0; 3];
            fill(mapping, &mut array);
            array
        });

        if lowercase.is_some() || uppercase.is_some() || titlecase.is_some() {
            offsets.insert(ch, u16::try_from(mappings.len()).unwrap());
            mappings.push((
                lowercase.unwrap_or([0; 2]),
                uppercase.unwrap_or([0; 3]),
                titlecase.unwrap_or([0; 3]),
            ));
        }
    }

    (mappings, offsets)
}

// If source is shorter than dest, it's assumed that the trailing values of dest are initialised
// to a suitable value (I.e. 0).
fn fill(source: &[u32], dest: &mut [u32]) {
    assert!(source.len() <= dest.len());
    for i in 0..source.len() {
        dest[i] = source[i];
    }
}

fn write_table(path: &Path, compiled_table: &CompiledTable) {
    let mut output =
        File::create(&path).expect(&format!("unable to open {}", path.to_string_lossy()));

    writeln!(output, "pub type Row = ([u32; 2], [u32; 3], [u32; 3]);").unwrap();

    writeln!(
        output,
        "\nconst LAST_CODEPOINT: u32 = 0x{:X};",
        compiled_table.last_code_point
    )
    .unwrap();
    writeln!(output, "\nconst BLOCK_SIZE: usize = {};", block::SIZE).unwrap();

    // Write out the records
    writeln!(
        output,
        "\nconst CASE_MAPPING_RECORDS: [Row; {}] = [",
        compiled_table.records.len()
    )
    .unwrap();
    for row in &compiled_table.records {
        writeln!(output, "    {:?},", row).unwrap();
    }
    write!(output, "];\n\n").unwrap();

    // Write out the blocks in address order
    writeln!(
        output,
        "\nconst CASE_MAPPING_BLOCKS: [u16; {}] = [",
        compiled_table.blocks.len() * block::SIZE
    )
    .unwrap();

    for (address, block) in &compiled_table.blocks {
        writeln!(output, "// BLOCK: {:04X}\n", address).unwrap();
        for (i, case_mapping) in block.iter().enumerate() {
            if i != 0 && (i & 0xF) == 0 {
                writeln!(output).unwrap();
            }

            write!(output, "{:?},", case_mapping).unwrap();
        }

        write!(output, "\n\n").unwrap();
    }
    writeln!(output, "];").unwrap();

    write!(output, "\n\n").unwrap();

    // Write out constants for the block offsets
    for (index, (address, _)) in compiled_table.blocks.iter().enumerate() {
        writeln!(
            output,
            "const BLOCK_OFFSET_{:04X}: u16 = 0x{:04X};",
            address,
            index * block::SIZE
        )
        .unwrap();
    }

    // Write out the array that maps case mapping to offsets
    writeln!(
        output,
        "\nconst CASE_MAPPING_BLOCK_OFFSETS: [u16; {}] = [",
        compiled_table.address_to_block_index.len()
    )
    .unwrap();
    for &(_, index) in &compiled_table.address_to_block_index {
        let (block_address, _) = compiled_table.blocks[index];
        writeln!(output, "    BLOCK_OFFSET_{:04X},", block_address).unwrap();
    }
    writeln!(output, "];").unwrap();
}

/// Lookup this code point in `table`
fn lookup(codepoint: u32, table: &'static [(u32, &'static [u32])]) -> Option<&'static [u32]> {
    table
        .binary_search_by(|&(cp, _)| {
            if codepoint < cp {
                Ordering::Greater
            } else if codepoint > cp {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        })
        .ok()
        .map(|idx| table[idx].1)
}

mod block {
    pub const SIZE: usize = 128;
    pub const LAST_INDEX: usize = SIZE - 1;

    use std::ops::{Index, IndexMut};

    /// A fixed size block
    ///
    /// Ideally this would be an array but that doesn't work until const generics are stabilised
    #[derive(Debug, PartialEq, Eq, Hash, Clone)]
    pub struct Block {
        data: Vec<u16>,
    }

    impl Block {
        pub fn new() -> Self {
            Block {
                data: vec![0; SIZE],
            }
        }

        pub fn reset(&mut self) {
            self.data.iter_mut().for_each(|val| *val = 0);
        }

        pub fn iter(&self) -> impl Iterator<Item = &u16> {
            self.data.iter()
        }
    }

    impl Index<usize> for Block {
        type Output = u16;

        fn index(&self, index: usize) -> &Self::Output {
            &self.data[index]
        }
    }

    impl IndexMut<usize> for Block {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            self.data.index_mut(index)
        }
    }
}
