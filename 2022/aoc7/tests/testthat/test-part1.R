test_that("cdup works", {
  expect_equal(cdup("/down/again"), "/down")
  expect_equal(cdup("/down"), "/")
  expect_equal(cdup("/"), "/")
})

test_that("walk_fs keeping track of cwd works", {
  fs <- init_fs()

  fs <- walk_fs(fs, "$ cd down")
  expect_equal(fs$cwd, "/down")
  fs <- walk_fs(fs, "$ cd again")
  expect_equal(fs$cwd, "/down/again")
  fs <- walk_fs(fs, "$ cd /")
  expect_equal(fs$cwd, "/")
  fs <- walk_fs(fs, "$ cd more")
  expect_equal(fs$cwd, "/more")
  fs <- walk_fs(fs, "$ cd abc")
  expect_equal(fs$cwd, "/more/abc")
  fs <- walk_fs(fs, "$ cd ..")
  expect_equal(fs$cwd, "/more")
})

test_that("walk_fs keeping track of sizes works", {
  fs <- init_fs()

  fs <- walk_fs(fs, "1000 ett")
  fs <- walk_fs(fs, "2000 två")
  fs <- walk_fs(fs, "$ cd down")
  fs <- walk_fs(fs, "3000 tre")
  fs <- walk_fs(fs, "4000 fyra")
  fs <- walk_fs(fs, "$ cd ..")
  fs <- walk_fs(fs, "$ cd again")
  fs <- walk_fs(fs, "5000 fem")
  fs <- walk_fs(fs, "6000 sex")

  expect_equal(fs$size[["/"]], 3000 + 7000 + 11000)
  expect_equal(fs$size[["/down"]], 7000)
  expect_equal(fs$size[["/again"]], 11000)
})

test_that("part1 works", {
  expect_equal(part1("simple.txt"), 95437)
  expect_equal(part1("input.txt"), 1390824)
})

test_that("part2 works", {
  expect_equal(part2("simple.txt"), 24933642)
  expect_equal(part2("input.txt"), 7490863)
})
