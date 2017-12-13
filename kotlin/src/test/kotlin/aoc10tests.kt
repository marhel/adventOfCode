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
            aoc10((0..255).toList(), listOf(76,1,88,148,166,217,130,0,128,254,16,2,130,71,255,229)) shouldBe(0)
        }
    }
}