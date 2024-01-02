Diff = under reverse (drop 1) - rotate 1.
Acc! = Diff dip ^2 : first .
Num = partition (parse regex "-?\\d+") != @\n .
Extrapolate!! = pop rows (box do ^2.2 (!=0 reduce + abs) flip 0 ^1 deshape)
ExFw = reduce + Extrapolate!!(Acc!+) reverse
ExBk = reduce + neg Extrapolate!!(Acc!-) identity

Num &fras $ input.txt
fork ExBk ExFw
