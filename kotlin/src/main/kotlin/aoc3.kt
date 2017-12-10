package adventOfCode

data class Point(val col: Int, val row: Int) {
    fun neighbors(): List<Point> {
        val ns = mutableListOf<Point>()
        for (colOffset in (-1..1)) {
            (-1..1)
                    .filter { colOffset != 0 || it != 0 }
                    .mapTo(ns) { Point(col + colOffset, row + it) }
        }
        return ns
    }
}

/*
    -4  -3  -2  -1   0.  1.  2   3   4
4   65  64  63  62  61  60  59  58  57
3   66  37  36  35  34  33  32  31  56
2   67  38  17  16  15  14  13  30  55
1   68  39  18   5   4   3  12  29  54
0   69  40  19   6   1   2! 11  28  53
-1  70  41  20   7   8   9  10! 27  52
-2  71  42  21  22  23  24  25  26! 51
-3  72  43  44  45  46  47  48  49  50!
-4  73  74  75  76  77  78  79  80  81  82!

*/
data class Spiral(val num: Int) {
    val side: Int = Math.ceil(Math.sqrt(num.toDouble())).toInt() / 2 * 2 + 1
    val offset: Int = (side - 1) / 2 - 1
    val start: Int = (side - 2) * (side - 2) + 1
    val index: Int = num - start
    val horiz = if (side < 2) 0 else Math.abs(((num - start) % (side - 1)) - offset)
    val distance: Int = offset + 1 + horiz
    val col: Int = when {
        num < 2 -> 0
        index < side - 1 -> {
            print("#c1"); offset + 1
        }
        index < (side - 1) * 2 -> {
            print("#c2"); (side - 1) * 2 - index - side / 2 - 1
        }
        index < (side - 1) * 3 -> {
            print("#c3"); -(offset + 1)
        }
        index <= side * side -> {
            print("#c4"); index - (side - 1) * 3 - side / 2 + 1
        }
        else -> -9999
    }
    val row: Int = when {
        num < 2 -> 0
        index < side - 1 -> {
            print("#r1"); index - side / 2 + 1
        }
        index < (side - 1) * 2 -> {
            print("#r2"); offset + 1
        }
        index < (side - 1) * 3 -> {
            print("#r3"); (side - 1) * 3 - index - side / 2 - 1
        }
        index <= side * side -> {
            print("#r4"); -(offset + 1)
        }
        else -> -9999
    }
    val coord: Point = Point(col, row)
    override fun toString(): String {
        return String.format("%d: %d %d @%s\n", num, side, index, coord)
    }

    val end: Int = side * side
}

fun aoc32() {
    val grid = mutableMapOf(Pair(Point(0, 0), 1))
    var lastScore = 0
    var index = 2
    while (lastScore < 312051) {
        val spiral = Spiral(index++)
        lastScore = spiral.coord.neighbors().fold(0, { acc, point -> grid.getOrDefault(point, 0) })
        grid.put(spiral.coord, lastScore)
    }
    return lastScore
}

fun spiral(max: Int): Pair<Int, Int> = Pair(max, Spiral(max).distance)
}

