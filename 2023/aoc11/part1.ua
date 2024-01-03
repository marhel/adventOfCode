Num = partition (= @# .) != @\n .
Expand = (
  dip (
    select
    select rise ...
    scan + =0 unwhere
  )
  # multiply with 1 for part 1, 999999 for part 2
  *
  + select :
)
SumPairs = /+ /+ select where scan + rows /* =[[0 0]] .
DistSum = /+ rows SumPairs abs cross - . transpose 

transpose where Num &fras $ input.txt

Expand 1 0 .
Expand 1 1 .:
Expand 999999 0 .:
Expand 999999 1 .:
dip (couple :) couple : dip pop
DistSum
: DistSum :
dip dip pop