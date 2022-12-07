library(readr)

crates <- function(s) {
	matches <- gregexpr(".(.).\\s", paste(s, " "))
	ndx <- map_dbl(matches[[1]], \(v) v + 1)
	values <- strsplit(s,"")[[1]][ndx]
}

read_crates <- function(input) {
	file.path("..", "..", input) %>%
	read_lines					 %>%
	head_while(\(l) substr(l, 2, 2) != "1") %>%
	map(crates)
}

to_stacks <- function(input) {
	read_crates(input) %>%
	transpose     %>%
	map(unlist)   %>%
	map(rev)      %>%
	map(function(L) {
		head_while(L, \(i) i != " ")
	})
}

get_move <- function(s) {
	r <- "move (\\d+) from (\\d) to (\\d)"
	regmatches(s, gregexec(r,s))[[1]][,1] %>% 
	tail(-1) %>%
	map_dbl(strtoi)
}

read_moves <- function(input) {
	file.path("..", "..", input) %>%
	read_lines					 %>%
	tail_while(\(l) l != "")     %>%
	map(get_move)
}

execute_move <- function(stacks, move, adapter_fn) {
	count = move[1]
	from = move[2]
	to = move[3]

	taken <- adapter_fn(tail(stacks[[from]], count))
	stacks[[to]] <- append(stacks[[to]], taken)
	stacks[[from]] <- head(stacks[[from]], -count)
	
	stacks
}

cratemover_9000 <- \(stacks, move) execute_move(stacks, move, rev)
cratemover_9001 <- \(stacks, move) execute_move(stacks, move, identity)

crane <- function(input, moving_fn) {
	stacks <- to_stacks(input)
	moves <- read_moves(input)

	Reduce(moving_fn, moves, stacks) %>%
	map(\(l) tail(l, 1)) 			 %>%	
	paste(collapse="")
}

part1 <- \(input) crane(input, cratemover_9000)
part2 <- \(input) crane(input, cratemover_9001)
