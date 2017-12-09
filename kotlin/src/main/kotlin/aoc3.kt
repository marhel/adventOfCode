package adventOfCode

data class Spiral(val num: Int) {
    val side: Int = Math.ceil(Math.sqrt(num.toDouble())).toInt() / 2 * 2 + 1
    val offset: Int = (side - 1) / 2 - 1
    val start: Int = (side - 2) * (side - 2) + 1
    val horiz = Math.abs(((num - start) % (side - 1)) - offset)
    val distance: Int = offset + 1 + horiz

    override fun toString(): String {
        return String.format("%d: %d %d %d %d-%d = %d\n", num, side, offset, horiz, start, end, distance)
    }

    val end: Int = side * side
}

fun g() = (2..50).map { i -> Spiral(i) }

fun spiral(max: Int): Pair<Int, Int> = Pair(max, Spiral(max).distance)
