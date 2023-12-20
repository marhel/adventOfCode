Grp ← ↘ 1 ⊜□ ¬ ∊, ":;," °□
Num ← ⋕ ⊢
Second ← ⊢ ↻ 1
Parse! ← (
  ⊃Second Num ⊜□ ¬ = @\s . °□
  ⊟ ^1 ⊗ : {"red" "green" "blue"}
)
One ← [/× ≤ ⊃⊢ Second ⍉ ∵Parse!(+ 12)]
Two ← [/× ⊕/↥ ⊃⊢ Second ⍉ ∵Parse!∘]
Part! ← ^1 Grp

&fras "input.txt"
⊜□ ≠ , @\n
/+ +1 ⊚ ∵Part!One .
/+ ∵Part!Two :

# Grp = drop 1 partition box not member, ":;," unbox
# Num = parse first 
# Second = first rotate 1
# Parse! = (
#  fork Second Num partition box not equals @\s duplicate unbox
#  couple ^1 indexof flip {"red" "green" "blue"} 
# )
# One = [reduce multiply <= fork first Second transpose each Parse!(add 12)]
# Two = [reduce multiply group reduce maximum fork first Second transpose each Parse!identity]
# Part! = ^1 Grp

# &fras "input.txt"
# partition box ≠ over @\n
# reduce ++1 where each Part!One duplicate
# reduce add each Part!Two flip
