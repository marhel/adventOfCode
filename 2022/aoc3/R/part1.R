library(readr)

part1 <- function(input) {
	file.path("..", "..", input) %>%
	read_lines					 %>%
	map_chr(common)				 %>%
	map_dbl(prio)				 %>%
	sum
}

charlist <- function(str) strsplit(str, "") %>% unlist

common <- function(contents) {
	l = nchar(contents)
	left = substr(contents, 1, l/2)
	right = substr(contents, l/2 + 1, l)
	intersect(charlist(left), charlist(right))
}

asc <- function(x) { strtoi(charToRaw(x), 16L) }
prio <- function(char) {
	a <- asc(char)
	if(a < 96) {
		a - 38
	} else {
		a - 96
	}
}

