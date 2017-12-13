package adventOfCode

fun swap(list: List<Int>, range: IntRange): List<Int> {
    if (range.last < list.size) {
        return list.slice(0 until range.first) + list.slice(range).reversed() + list.slice((range.last+1) until list.size)
    } else {
        // join end + beginning
        val end = list.slice((range.first) until list.size)
        val midRange = (range.last % list.size + 1) until (range.first)
        val mid = list.slice(midRange)
        val begRange = 0 until (range.last % list.size + 1)
        val beg = list.slice(begRange)
        val res = (end + beg).reversed()
        val newbeg = res.slice(end.size until res.size)
        val newend = res.slice(0 until end.size)
        return newbeg + mid + newend
    }
}

fun aoc10(initList: List<Int>, swaps: List<Int>): Int {
    var pc = 0
    var skip = 0
    val result = swaps.fold(initList, {list, len ->
        val res = swap(list, pc..(pc + len - 1))
        pc = (pc + len + skip++) % initList.size
        res
    })
    return result[0] * result[1]
}
