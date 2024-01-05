Hash = fold (mod 256 *17 + - @\0) : 0
PartOne = /+ partition Hash != @, .
Label = un box first first regex "[a-z]+"
Focal = parse regex "\\d+"
Cmd = = @= first first first regex "[-=]"
Clear = box keep != :dip: rows (pick 1) . unbox
Prep = dip fork ggidi : dipdip (first rotate 2) : first rotate 3 : first rotate 1 ..
RemoveLens =|2.1 (
  under select Clear dipdip pop Prep 
)
Insert = (
  unbox 
  dip (first reverse first .)
  rows (pick 1) .
  :
  dip (find :)
  where:
  (join pop|under select pop) length .  
  box 
)
AddLens =|2.1 (
  under select Insert dipdip [couple] Prep    
)
Process = (
  (RemoveLens|AddLens) first .
)
Parse = (
  partition box != @, .
  each (Hash Label) . 
  classify each (box Label) . :
  deshape fill 0 rows Focal . :
  each Cmd . :
  pop :
  transpose join join couple
  # Array of [Operation Value Index Hash]
)
Plan = box rows (joinjoin) unbox first : dip : +1 range length . rows pick 0 unbox first reverse .
Analyze = (
  fold Process : reshape 256 box reshape 0_2 0
  couple +1 range length .
  transpose
  rows Plan
)
Power = /+ rows /* unbox
PartTwo = /+ each Power Analyze Parse
Read = take where find @\n . &fras $ input.txt

Read
&s PartOne .
&s PartTwo
