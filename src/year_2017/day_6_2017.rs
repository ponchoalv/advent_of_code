use std::cmp::Ordering;
use std::collections::HashSet;
use std::iter::FusedIterator;

fn get_vector_from_input(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .map(|value| value.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}

struct MemoryBlocksIterator {
    current_blocks: Vec<usize>,
    blocks_memory: HashSet<Vec<usize>>,
}

impl MemoryBlocksIterator {
    fn new(blocks: Vec<usize>) -> MemoryBlocksIterator {
        let mut blocks_memory: HashSet<Vec<usize>> = HashSet::new();
        blocks_memory.insert(blocks.clone());

        MemoryBlocksIterator {
            current_blocks: blocks,
            blocks_memory,
        }
    }

    fn get_max(&self) -> (usize, usize) {
        let (index, &value) = self
            .current_blocks
            .iter()
            .enumerate()
            .max_by(|(_x, y), (_z, k)| {
                if y >= k {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            })
            .unwrap();
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
        }
    }
}

impl Iterator for MemoryBlocksIterator {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        self.cycle_blocks();
        if self.blocks_memory.insert(self.current_blocks.clone()) {
            Some(self.current_blocks.clone())
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

pub fn day_6_2_2017(input: &str) -> usize {
    let memory_blocks = get_vector_from_input(input);
    let last_vector = MemoryBlocksIterator::new(memory_blocks).last().unwrap();

    MemoryBlocksIterator::new(last_vector).count() + 1
}

#[test]
fn probando_day_6_1_2017() {
    let input = "0 2 7 0";
    let result = day_6_1_2017(input);
    assert_eq!(5, result);

    let input = "0	5	10	0	11	14	13	4	11	8	8	7	1	4	12	11";
    let result = day_6_1_2017(input);
    assert_eq!(7864, result);
}

#[test]
fn probando_day_6_2_2017() {
    let input = "0 2 7 0";
    let result = day_6_2_2017(input);
    assert_eq!(4, result);

    let input = "0	5	10	0	11	14	13	4	11	8	8	7	1	4	12	11";
    let result = day_6_2_2017(input);
    assert_eq!(1695, result);
}
