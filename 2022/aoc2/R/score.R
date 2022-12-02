library(hash)

itemscore <- hash(c("A", "B", "C", "X", "Y", "Z"), c(1:3, 1:3))
gamescore <- hash(c("X", "Y", "Z"), list(c(3,0,6), c(6,3,0), c(0,6,3)))

# not sure using lookup tables is "better"
score_alt <- function(rule) {
	# A = Rock, B = Paper, C = Scissors
	# X = Rock, Y = Paper, Z = Scissors

	comp <- strsplit(rule, " ") %>% unlist
	they <- itemscore[[comp[1]]]
	me <- itemscore[[comp[2]]]
	gamescore[[comp[2]]][they] + me
}

score <- function(rule) {
	# A = Rock, B = Paper, C = Scissors
	# X = Rock, Y = Paper, Z = Scissors
	# score (1 for Rock, 2 for Paper, and 3 for Scissors)	
	switch(rule,
		"A Y" = 6+2,
		"B Y" = 3+2,
		"C Y" = 0+2,
		"A X" = 3+1,
		"B X" = 0+1,
		"C X" = 6+1,
		"A Z" = 0+3,
		"B Z" = 6+3,
		"C Z" = 3+3,
		)
}

score2 <- function(rule) {
	# A = Rock, B = Paper, C = Scissors
	# X = Lose, Y = Draw, Z = Win
	# score (1 for Rock, 2 for Paper, and 3 for Scissors)
	switch(rule,
		"A Y" = 3+1,
		"B Y" = 3+2,
		"C Y" = 3+3,
		"A X" = 0+3,
		"B X" = 0+1,
		"C X" = 0+2,
		"A Z" = 6+2,
		"B Z" = 6+3,
		"C Z" = 6+1,
		)
}
