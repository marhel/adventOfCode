library(readr)
library(purrr)

calc <- function(input, scorer) {
	# this is the same as
	#   sum(map_dbl(read_lines(file.path("..", "..", input)), scorer)
	# but pipelined

	file.path("..", "..", input) %>%
	read_lines  				 %>%
	map_dbl(scorer)				 %>%
	sum
}

part1 <- function(input) calc(input, score)
part2 <- function(input) calc(input, score2)
