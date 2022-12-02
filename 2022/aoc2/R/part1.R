library(readr)
library(purrr)

calc <- function(input, scorer) {
	lines <- read_lines(file.path("..", "..", input))

	Reduce("+", map(lines, scorer))
}

part1 <- function(input) calc(input, score)
part2 <- function(input) calc(input, score2)
