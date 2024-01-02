Diff = under reverse (drop 1) - rotate 1.
Acc! = Diff dip ^2 : first .
Num = partition (parse regex "-?\\d+") != @\n .
ExtrapolateFw = reduce +     pop rows (box do Acc!+ (!=0 reduce + abs) flip 0 reverse deshape)
ExtrapolateBk = reduce + neg pop rows (box do Acc!- (!=0 reduce + abs) flip 0         deshape)

Num &fras $ input.txt
fork ExtrapolateBk ExtrapolateFw 
