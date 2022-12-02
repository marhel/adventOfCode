test_that("score works", {
  expect_equal(score("A Y"), 8)
  expect_equal(score("B X"), 1)
  expect_equal(score("C Z"), 6)
})

test_that("score2 works", {
  expect_equal(score2("A Y"), 4)
  expect_equal(score2("B X"), 1)
  expect_equal(score2("C Z"), 7)
})
