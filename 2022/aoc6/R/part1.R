library(readr)
library(purrr)

step_slide <- function(size) {
  \(a, v) {
    a$vec <- c(tail(a$vec, size-1), v)
    if(length(a$vec) >= size)
    {
      a$vecvec[[length(a$vecvec) + 1]] <- a$vec
    }
    a
  }
}

slide_window = function(l, size) {
  a <- Reduce(step_slide(size), l, list())
  a$vecvec
}

charlist <- function(str) strsplit(str, "") %>% unlist

uniq <- function(N) {
  \(L) length(unique(L)) == N
}

found_after <- function(signal, N)
{
  signal                       %>%
  charlist                     %>%
  # create sliding window of size N
  slide_window(N)              %>%
  # sapply is 'map', is number of unique entries in window == N?
  sapply(uniq(N))              %>%
  # find TRUE indices
  which()                      %>%
  # pick first
  head(1) + N - 1
}

calc <- function(input, N) {
  file.path("..", "..", input) %>%
  read_lines                   %>%
  head(1)                      %>%
  found_after(N)
}

part1 <- \(input) calc(input, N=4)
part2 <- \(input) calc(input, N=14)
