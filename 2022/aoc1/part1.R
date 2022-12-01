library(readr)
items <- as.numeric(read_lines("simple.txt"))

addOrAppend <- function(a, v) 
	if(is.na(v)) { 
		c(a, 0)
	} else { 
		l <- length(a)
		a[l] <- a[l] + v
		a
	}

calories <- Reduce(addOrAppend, items, c(0))
max_cal = max(calories)

# part 2
top_3_sum = sum(head(sort(calories, decreasing=TRUE), 3))
