test_that("step_slide works", {
  a <- Reduce(step_slide(4), 1:10, list())
  expect_equal(a$vecvec[[1]], c(1,2,3,4))
  expect_equal(a$vecvec[[2]], c(2,3,4,5))
  expect_equal(a$vecvec[[3]], c(3,4,5,6))
  expect_equal(a$vecvec[[4]], c(4,5,6,7))
  expect_equal(a$vecvec[[5]], c(5,6,7,8))
  expect_equal(a$vecvec[[6]], c(6,7,8,9))
  expect_equal(a$vecvec[[7]], c(7,8,9,10))
})

test_that("slide_window works", {
  a <- slide_window(1:10, 4)

  expect_equal(a[[1]], c(1,2,3,4))
  expect_equal(a[[2]], c(2,3,4,5))
  expect_equal(a[[3]], c(3,4,5,6))
  expect_equal(a[[4]], c(4,5,6,7))
  expect_equal(a[[5]], c(5,6,7,8))
  expect_equal(a[[6]], c(6,7,8,9))
  expect_equal(a[[7]], c(7,8,9,10))
})

test_that("marker_found_after works", {
  marker_found_after <- partial(found_after, N=4)
  expect_equal(marker_found_after("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7)
  expect_equal(marker_found_after("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5)
  expect_equal(marker_found_after("nppdvjthqldpwncqszvftbrmjlhg"), 6)
  expect_equal(marker_found_after("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10)
  expect_equal(marker_found_after("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11)
})

test_that("message_found_after works", {
  message_found_after <- partial(found_after, N=14)
  expect_equal(message_found_after("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19)
  expect_equal(message_found_after("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23)
  expect_equal(message_found_after("nppdvjthqldpwncqszvftbrmjlhg"), 23)
  expect_equal(message_found_after("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29)
  expect_equal(message_found_after("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26)
})

test_that("part1 works", {
  expect_equal(part1("simple.txt"), 7)
  expect_equal(part1("input.txt"), 1892)
})

test_that("part2 works", {
  expect_equal(part2("simple.txt"), 19)
  expect_equal(part2("input.txt"), 2313)
})