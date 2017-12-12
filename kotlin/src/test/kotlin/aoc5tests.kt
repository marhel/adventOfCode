package adventOfCode

import io.kotlintest.matchers.shouldBe
import io.kotlintest.specs.StringSpec

class JumpTests : StringSpec() {
    init {
        val table = intArrayOf(0, 3, 0, 1, -3)
        "can jump" {
            jump(0, table) shouldBe(0)
            table shouldBe(intArrayOf(1, 3, 0, 1, -3))
        }
        "can jump away" {
            jump(0, table) shouldBe(0)
            jump(0, table) shouldBe(1)
            table shouldBe(intArrayOf(2, 3, 0, 1, -3))
        }
        "can jump away to the end" {
            jump(0, table) shouldBe(0)
            jump(0, table) shouldBe(1)
            jump(1, table) shouldBe(4)
            table shouldBe(intArrayOf(2, 4, 0, 1, -3))
        }
        "can jump backwards" {
            jump(0, table) shouldBe(0)
            jump(0, table) shouldBe(1)
            jump(1, table) shouldBe(4)
            jump(4, table) shouldBe(1)
            table shouldBe(intArrayOf(2, 4, 0, 1, -2))
        }
        "can jump out" {
            jump(0, table) shouldBe(0)
            jump(0, table) shouldBe(1)
            jump(1, table) shouldBe(4)
            jump(4, table) shouldBe(1)
            jump(1, table) shouldBe(5)
            table shouldBe(intArrayOf(2, 5, 0, 1, -2))
        }
        "can do that out" {
            val jumps = """0
3
0
1
-3
"""
            jumper(jumps) shouldBe(5)
        }
    }
}