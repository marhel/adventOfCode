package adventOfCode

import io.kotlintest.forAll
import io.kotlintest.matchers.shouldBe
import io.kotlintest.properties.forAll
import io.kotlintest.properties.headers
import io.kotlintest.properties.row
import io.kotlintest.properties.table
import io.kotlintest.specs.StringSpec

class SpiralTests : StringSpec() {
    var banks2hot = arrayOf(14, 0, 15, 12, 11, 11, 3, 5, 1, 6, 8, 4, 9, 1, 8, 4)

    val coordinates = table(
            headers("pos", "col", "row"),
            row(1, 0, 0),
            row(2, 1, 0),
            row(3, 1, 1),
            row(4, 0, 1),
            row(5, -1, 1),
            row(6, -1, 0),
            row(7, -1, -1),
            row(8, 0, -1),
            row(9, 1, -1),
            row(10, 2, -1),
            row(11, 2, 0),
            row(12, 2, 1),
            row(13, 2, 2),
            row(14, 1, 2),
            row(15, 0, 2),
            row(16, -1, 2),
            row(17, -2, 2),

            row(18, -2, 1),
            row(19, -2, 0),
            row(20, -2, -1),
            row(21, -2, -2),

            row(22, -1, -2),
            row(23, 0, -2),
            row(24, 1, -2),
            row(25, 2, -2),

            row(28, 3, 0),
            row(34, 0, 3),
            row(35, -1, 3),
            row(36, -2, 3),
            row(39, -3, 1),
            row(47, 1, -3),
            row(50, 4, -3),
            row(64, -3, 4),
            row(65, -4, 4),
            row(66, -4, 3),

            row(76, -1, -4),
            row(82, 5, -4)
    )

    init {
        "aoc 3.1 distance" {
            val myTable = table(
                    headers("pos", "distance"),
                    row(1, 0),
                    row(12, 3),
                    row(23, 2),
                    row(1024, 31),
                    row(312051, 430)
            )
            forAll(myTable, { pos, distance ->
                Spiral(pos).distance shouldBe(distance)
            })
        }
        "aoc 3.1 pos 12" {
            Spiral(12).distance shouldBe(3)
        }
        "aoc 3.1 pos 23" {
            Spiral(23).distance shouldBe(2)
        }
        "aoc 3.1 pos 1024" {
            Spiral(1024).distance shouldBe(31)
        }
        "aoc 3.1 pos 1" {
            Spiral(312051).distance shouldBe(430)
        }
        forAll(coordinates, { pos, col, row ->
            "aoc 3.2 coord pos $pos should have col $col" {
                Spiral(pos).coord.col shouldBe(col)
            }
        })
        forAll(coordinates, { pos, col, row ->
        "aoc 3.2 coord pos $pos should have row $row" {
                Spiral(pos).coord.row shouldBe(row)
            }
        })

        "aoc 3.2 matches examples" {
            val answers = listOf(1, 1, 2, 4, 5, 10, 11, 23, 25, 26, 54, 57, 59, 122, 133, 142, 147, 304, 330, 351, 362, 747, 806)
            // (2..25).map { Spiral(it) } should be
            aoc32().take(answers.size).toList() shouldBe(answers)
        }

        "aoc 3.2 can generate the answer" {
            aoc32().dropWhile { it < 312051 }.first() shouldBe(312453)
        }
    }
}