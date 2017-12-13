package adventOfCode

import io.kotlintest.matchers.shouldBe
import io.kotlintest.specs.StringSpec


class SwapTests : StringSpec() {
    init {
        val swaps = listOf(3, 4, 1, 5)
        val init = (0..4).toList()
        val init2 = (0..18).toList()

        "swapping works" {
            swap(init, 0..2) shouldBe (listOf(2, 1, 0, 3, 4))
        }

        "swapping across the end of the list works" {
            swap(init, 3..5) shouldBe (listOf(3, 1, 2, 0, 4))
        }
        "swapping across the end of a larger list works" {
            swap(init2, 17..21) shouldBe (listOf(0, 18, 17, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 2, 1))
        }

        "aoc10 sample works" {
            aoc10(init, swaps) shouldBe (12)
        }

        "aoc10.1 real input works" {
            aoc10((0..255).toList(), listOf(76,1,88,148,166,217,130,0,128,254,16,2,130,71,255,229)) shouldBe(29240)
        }

        "aoc 10.2 sample input" {
            knothash("") shouldBe ("a2582a3a0e66e6e86e3812dcb672a272")
            knothash("AoC 2017") shouldBe ("33efeb34ea91902bb2f59c9920caa6cd")
            knothash("1,2,3") shouldBe ("3efbe78a8d82f29979031a4aa0b16a9d")
            knothash("1,2,4") shouldBe ("63960835bcdc130f0b66d7ff4f6a5a8e")
        }
        "aoc 10.2 real input" {
            var code = "76,1,88,148,166,217,130,0,128,254,16,2,130,71,255,229"
            knothash(code) shouldBe("4db3799145278dc9f73dcdbc680bd53d")
        }
    }
}