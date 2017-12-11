package adventOfCode

import io.kotlintest.matchers.shouldBe
import io.kotlintest.specs.StringSpec

val code = """5 9 2 8
9 4 7 3
3 8 6 5"""

class SpreadsheetTests : StringSpec() {
    init {
        "should find evenly divisible" {
            checksum2(code) shouldBe (9)
        }
        "should find evenly divisible in spreadsheet" {
            checksum2(spreadsheet) shouldBe (246)
        }
    }
}

