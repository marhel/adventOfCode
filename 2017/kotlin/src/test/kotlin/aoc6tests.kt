package adventOfCode

import io.kotlintest.matchers.shouldBe
import io.kotlintest.specs.StringSpec

class MyTests : StringSpec() {
    var banks2hot = arrayOf(14,	0,	15,	12,	11,	11,	3,	5,	1,	6,  8,	4,	9,	1,	8,	4)
    init {
        "aoc 6.2 real example (cold)" {
            var banks2 = arrayOf(14,	0,	15,	12,	11,	11,	3,	5,	1,	6,  8,	4,	9,	1,	8,	4)
            reallocate2(banks2) shouldBe (11137 - 10100)
        }

//        "aoc 6.2 real example * 500" {
//            for(i in (0..500)) {
//                var banks2 = arrayOf(14,	0,	15,	12,	11,	11,	3,	5,	1,	6,  8,	4,	9,	1,	8,	4)
//                reallocate2(banks2) shouldBe (11137 - 10100)
//            }
//        }
//
//        "aoc 6.2 real example 2 (hot)" {
//            reallocate2(banks2hot) shouldBe (11137 - 10100)
//        }

        "banks example" {
            var banks: Array<Int> = arrayOf(0, 2, 7, 0)
            reallocate(banks) shouldBe 5
        }

        "banks example2" {
            var banks: Array<Int> = arrayOf(0, 2, 7, 0)
            reallocate2(banks) shouldBe 4
        }

    }
}
