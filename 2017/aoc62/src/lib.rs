#![feature(test)]
extern crate test;

fn max_block(banks: &Vec<u32>) -> (usize, u32) {
    banks.iter().enumerate().fold((99999999usize, 0), |acc, item| if acc.1 >= *item.1 { acc } else {(item.0, *item.1)})
}

fn redistribute(banks: &mut Vec<u32>, block: (usize, u32)) -> Vec<u32> {
    banks[block.0] = 0;
    let len = banks.len() as u32;
    let all = block.1 / len;
    let some = block.1 % len;
    for elem in banks.iter_mut() {
        *elem += all;
    }
    for it in (0..some).map(|it|(it + block.0 as u32 + 1) % len) {
        banks[it as usize] += 1;
    }
    banks.clone()
}

use std::collections::HashMap;

fn reallocate2(banks: &mut Vec<u32>) -> u32 {
    let mut indices: HashMap<Vec<u32>, u32> = HashMap::new();
    let mut turns = 0u32;
    loop {
        let max = max_block(banks);
        let current = redistribute(banks, max);
        // println!("{:?}", current);
        if indices.contains_key(&current) {
            return turns - indices.get(&current).unwrap();
        }
        indices.insert(current, turns);
        turns += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn max_block_works() {
        assert_eq!((2, 7), max_block(&vec![0u32, 2u32, 7u32, 0u32]));
    }

    #[test]
    fn redistribute_works() {
        assert_eq!(vec![2, 4, 1, 2], redistribute(&mut vec![0u32, 2u32, 7u32, 0u32], (2, 7)));
    }

    #[test]
    fn reallocate2_works() {
        assert_eq!(4, reallocate2(&mut vec![0u32, 2u32, 7u32, 0u32]));
    }

    #[test]
    fn real_reallocate2_works() {
        let mut banks = vec![14u32,   0u32,  15u32, 12u32, 11u32, 11u32, 3u32,  5u32,  1u32,  6u32,  8u32,  4u32,  9u32,  1u32,  8u32,  4u32];
        assert_eq!((11137 - 10100), reallocate2(&mut banks));
    }
 
    #[bench]
    fn bench_max_block(b: &mut Bencher) {
        b.iter(|| {
            {
                let mut banks = vec![14u32,   0u32,  15u32, 12u32, 11u32, 11u32, 3u32,  5u32,  1u32,  6u32,  8u32,  4u32,  9u32,  1u32,  8u32,  4u32];
                let v = reallocate2(&mut banks);
                assert_eq!((11137 - 10100), v);
                v
          }
        });
    }
}

/*
fun maxBlock(banks: Array<Int>): IndexedValue<Int> = banks.withIndex().reduce { max, item ->
    if (item.value > max.value) {
        item
    } else {
        max
    }
}

fun redistribute(banks: Array<Int>, block: IndexedValue<Int>): String {
    banks[block.index] = 0
    val all = block.value / banks.size
    val some = block.value % banks.size
    (0 until banks.size).forEach { banks[it] += all}
    (0 until some)
            .map { (it + block.index + 1) % banks.size }
            .forEach { banks[it] += 1 }
    return banks.joinToString("")
}

fun reallocate(banks: Array<Int>): Int {
    val seen = mutableSetOf<String>()
    var turns = 1
    while(true) {
        val current = redistribute(banks, maxBlock(banks))
        if(seen.contains(current))
            break;
        turns++
        seen.add(current)
    }
    return turns
}


fun reallocate2(banks: Array<Int>): Int {
    val indices = mutableMapOf<String, Int>()
    var turns = 0
    while(true) {
        val current = redistribute(banks, maxBlock(banks))
        if(indices.contains(current))
            return turns - indices.getOrDefault(current,0)
        indices[current] = turns++
    }
    return -1
}
*/