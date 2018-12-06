use std::cmp::Ordering;
use std::collections::HashSet;
use std::iter::FusedIterator;

fn get_vector_from_input(input: &str) -> Vec<usize> {
    input.split_whitespace().map(|value| {
        value.parse::<usize>().unwrap()
    }).collect::<Vec<usize>>()
}

fn get_max(blocks: &[usize]) -> (usize, usize) {
    let (index, &value) = blocks.iter().enumerate().max_by(|(_x, y), (_z, k)| {
        if y >= k {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }).unwrap();
    (index, value)
}

fn cycle_blocks(blocks: Vec<usize>, max: (usize, usize)) -> Vec<usize> {
    let mut blocks = blocks.clone();
    let len = blocks.len();
    let start = 1 + max.0;
    let end = max.1 + max.0 + 1;

    blocks[max.0] = 0;

    for i in start..end {
        let index = i % len;
        blocks[index] += 1;
        println!("index {:?} and Blocks {:?}", index, blocks);
    }

    blocks
}

struct MemoryBlocksIterator {
    current_blocks: Vec<usize>,
    blocks_memory: HashSet<Vec<usize>>,
}

impl MemoryBlocksIterator {
    fn new(blocks: Vec<usize>) -> MemoryBlocksIterator {
        let len  = blocks.len();
        let mut blocks_memory: HashSet<Vec<usize>> = HashSet::with_capacity(len * 3);
        blocks_memory.insert(blocks.clone());

        MemoryBlocksIterator {
            current_blocks: blocks,
            blocks_memory,
        }
    }

    fn get_max(&self) -> (usize, usize) {
        let (index, &value) = self.current_blocks.iter().enumerate().max_by(|(_x, y), (_z, k)| {
            if y >= k {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        }).unwrap();
        (index, value)
    }

    fn cycle_blocks(&mut self) {
        let len = self.current_blocks.len();
        let max = self.get_max();

        let start = 1 + max.0;
        let end = max.1 + max.0 + 1;

        self.current_blocks[max.0] = 0;

        for i in start..end {
            let index = i % len;
            self.current_blocks[index] += 1;
            println!("index {:?} and Blocks {:?}", index, self.current_blocks);
        }
    }

}

impl Iterator for MemoryBlocksIterator {
    type Item = bool;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        self.cycle_blocks();
        if self.blocks_memory.insert(self.current_blocks.clone()) {
            Some(true)
        } else {
            None
        }
    }
}

impl FusedIterator for MemoryBlocksIterator {}

pub fn day_6_1_2017(input: &str) -> usize {
    let memory_blocks = get_vector_from_input(input);
    let memory_cycling_iterator = MemoryBlocksIterator::new(memory_blocks);

    memory_cycling_iterator.count() + 1
}

#[test]
fn probando_day_6_1_2017() {
    let input = "0 2 7 0";
    let vector = get_vector_from_input(input);
    let len = vector.len();
    println!("Vector {:?}", vector);

    let mut set_of_vec: HashSet<Vec<usize>> = HashSet::with_capacity(len);
    println!("Insertando vector {:?} operacion fue {:?}",vector,set_of_vec.insert(vector.clone()));


    let max = get_max(&vector);
    println!("Max of Vector {:?}", max);

    let cycled = cycle_blocks(vector.clone(), max);
    println!("Cycled Vector {:?}", cycled);

    println!("Insertando vector {:?} operacion fue {:?}",cycled,set_of_vec.insert(cycled.clone()));
    println!("Insertando vector {:?} operacion fue {:?}",vector,set_of_vec.insert(vector.clone()));

    let input = "0	5	10	0	11	14	13	4	11	8	8	7	1	4	12	11";
    let vector = get_vector_from_input(input);
    println!("Vector {:?}", vector);
    println!("Max of Vector {:?}", get_max(&vector));

    let input = "0	5	10	0	11	14	13	4	11	14	8	7	1	4	12	11";
    let vector = get_vector_from_input(input);
    println!("Vector {:?}", vector);
    println!("Max of Vector {:?}", get_max(&vector));

    let input = "3 1 2 3";
    let vector = get_vector_from_input(input);
    println!("Vector {:?}", vector);
    println!("Max of Vector {:?}", get_max(&vector));


    println!("Starting Count");
    let input = "0 2 7 0";
    let vector = get_vector_from_input(input);
    let memory_clycle_iterator = MemoryBlocksIterator::new(vector);

    let count = memory_clycle_iterator.count() + 1;
    println!("Steps taken for looped cycle: {:?}", count);

}