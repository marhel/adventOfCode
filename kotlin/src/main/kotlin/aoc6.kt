package adventOfCode

// var banks = arrayOf(14,	0,	15,	12,	11,	11,	3,	5,	1,	6,  8,	4,	9,	1,	8,	4)
/*

The banks start with 0, 2, 7, and 0 blocks.

The third bank has the most blocks, so it is chosen for redistribution.

Starting with the next bank (the fourth bank) and then continuing to the first
bank, the second bank, and so on, the 7 blocks are spread out over the memory
banks. The fourth, first, and second banks get two blocks each, and the third
bank gets one back. The final result looks like this: 2 4 1 2.

Next, the second bank is chosen because it contains the most blocks (four).
Because there are four memory banks, each gets one block. The result is: 3 1 2
3.

Now, there is a tie between the first and fourth memory banks, both of which
have three blocks. The first bank wins the tie, and its three blocks are
distributed evenly over the other three banks, leaving it with none: 0 2 3 4.

The fourth bank is chosen, and its four blocks are distributed such that each
of the four banks receives one: 1 3 4 1.

The third bank is chosen, and the same thing happens: 2 4 1 2.

*/
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
    return banks.joinToString(", ")
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
        // println(current + "\n")
        if(indices.contains(current))
            return turns - indices.getOrDefault(current,0)
        indices[current] = turns++
    }
    return -1
}
