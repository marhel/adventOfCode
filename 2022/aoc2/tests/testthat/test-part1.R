test_that("part1 works", {
  expect_equal(part1("simple.txt"), 15)
  expect_equal(part1("input.txt"), 9759)
})
test_that("part2 works", {
  expect_equal(part2("simple.txt"), 12)
  expect_equal(part2("input.txt"), 12429)
})
