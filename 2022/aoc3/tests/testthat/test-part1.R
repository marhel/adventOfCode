test_that("part1 works", {
  expect_equal(part1("simple.txt"), 157)
  expect_equal(part1("input.txt"), 0)
})

test_that("common works", {
  expect_equal(common("vJrwpWtwJgWrhcsFMMfFFhFp"), "p")
  expect_equal(common("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"), "L")
  expect_equal(common("PmmdzqPrVvPwwTWBwg"), "P")
})

test_that("prio works", {
  expect_equal(prio("p"), 16)
  expect_equal(prio("L"), 38)
  expect_equal(prio("P"), 42)
})
