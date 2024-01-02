F! = first where rows (/* ^1 repeat first 3)
Scan = (
    flip dip (+ 1)
    dip (flip dip (pick dip.) rotate 1 flip join flip first over)
    flip
)

&fras $ input.txt
partition box != @\n .
fork drop (indexof : "LR" unbox first take) 1
rows (regex "[A-Z]{3}")
F!(= "ZZZ") .
ZI = 
F!(= "AAA") .
AI = 
reshape [neg 1 3] classify deshape
..
bracket (first pick ZI) (first pick AI)
ZZZ=
AAA=
rows drop 1 select rise .
:
0
AAA

do Scan (!= ZZZ)
flip
dip (pop pop pop)
&s $"Going from _ (row _) to _ (row _) in _ steps" AAA AI ZZZ ZI

# # dip pop ? pop
# # flip dip over 
# # :
# # indexof "LR"
# # # fold
# # # shape
# # # member : "\\n\\n"
# # # /+
# # # fork (regex "[A-Z]{3}\\b") (regex "[RL]+") 
# # # reshape 
# # # take 7 
# # # $ LLRRLRRRLLRLRRLLLLRLRRLRR
# # [1 2 3 4]
# # under rotate first
