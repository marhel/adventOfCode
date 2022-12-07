library(hash)

init_fs <- function() {
    fs <- list(cwd = "/", size = hash("/", 0))
}

cdup <- function(path) {
  if(path == "/") return("/")
  up <- paste(head(strsplit(path, "/") %>% unlist, -1), collapse="/")
  if(up == "") return("/")
  up
}

handle_cd <- function(fs, path) {
  if(path == "/") {
    fs$cwd <- "/"
  } else if(path == "..") {
    fs$cwd <- cdup(fs$cwd)
  } else {
    fs$cwd <- paste(if(fs$cwd == "/") "" else fs$cwd, path, sep="/")
  }
  if(is.null(fs$size[[fs$cwd]]))
    fs$size[[fs$cwd]] <- 0
  fs
}

grow_branch <- function(fs, size) {
  cwd <- fs$cwd
  repeat {
    fs$size[[cwd]] <- fs$size[[cwd]] + size
    if(cwd == "/") break
    cwd <- cdup(cwd)
  }
}

walk_fs <- function(fs, entry) {
	w <- strsplit(entry, " ") %>% unlist
  if(w[1] == "$") {
    if(w[2] == "cd") {
      fs <- handle_cd(fs, w[3])
    }
  } else {
    size <- strtoi(w[1])
    if(!is.na(size)) {
      grow_branch(fs, size)
    }
  }
  fs
}

fswalker <- function(L) Reduce(walk_fs, L, init_fs())

part1 <- function(input) {
  fs <- file.path("..", "..", input) %>%
        read_lines                   %>%
        fswalker()

  sizes <- as.list(fs$size)

  sizes[which(sizes < 1e5)] %>%
  unlist                       %>%
  sum
}

part2 <- function(input) {
  fs <- file.path("..", "..", input) %>%
        read_lines                   %>%
        fswalker()

  sizes <- as.list(fs$size) %>% unname
  free <- 7e7 - fs$size[["/"]]
  needed <- 3e7 - free

  sizes[which(sizes >= needed)] %>%
  unlist                        %>%
  sort                          %>%
  head(1)
}
