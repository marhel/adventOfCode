package adventOfCode

data class Spiral(val start: Int, val end: Int, val side: Int, val offset: Int)

fun squares(max: Int): Int {
    return (listOf(0, 0) + (3..(Math.sqrt(max.toDouble()).toInt()+1) step 2).map {side -> Spiral((side-2) * (side-2) + 1, side*side, side, (side-1) / 2 - 1) }.flatMap{ s ->
        val offsets = ((1..s.offset).reversed() + (0..s.offset+1)).toIntArray()
        (s.start..s.end).map  { k ->
            s.offset+1 + offsets[(k-s.start) % offsets.size]
        }
    })[max]
}
