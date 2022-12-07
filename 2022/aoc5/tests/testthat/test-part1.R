test_that("crates extraction works", {
l <- c(
"    [D]    ",
"[N] [C]    ",
"[Z] [M] [P]",
" 1   2   3 ")

  expect_equal(crates(l[1]), c(" ", "D", " "))
  expect_equal(crates(l[2]), c("N", "C", " "))
  expect_equal(crates(l[3]), c("Z", "M", "P"))
  expect_equal(crates(l[4]), c("1", "2", "3"))
})

test_that("crate_lines works", {
  expect_equal(length(read_crates("simple.txt")), 3)  
  })

test_that("part1 works", {
    expect_equal(part1("simple.txt"), "CMZ")
    expect_equal(part1("input.txt"), "BWNCQRMDB")
  })

test_that("part2 works", {
    expect_equal(part2("simple.txt"), "MCD")
    expect_equal(part2("input.txt"), "")
  })
