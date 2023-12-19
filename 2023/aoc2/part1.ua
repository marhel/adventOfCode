Grp ← ↘ 1 ⊜□ ¬ ∊, ":;," °□
Num ← ⋕ ⊢
Second ← ⊢ ↻ 1
Parse ← (
  ⊃Second Num ⊜□ ¬ = @\s . °□
  ⊟ +12 ⊗ : {"red" "green" "blue"}
)
Run ← [/× ≤ ⊃⊢ Second ⍉ ∵Parse]
PartOne ← Run Grp

&fras "input.txt"
⊜□ ≠ , @\n
/++1 ⊚ ∵PartOne

# Grp = drop 1 partition box not member, ":;," unbox
# Num = parse first 
# Second = first rotate 1
# Parse = (
#  fork Second Num partition box not equals @\s duplicate unbox
#  couple add 12 indexof flip {"red" "green" "blue"} 
# )
# Run = [reduce multiply <= fork first Second transpose each Parse]
# PartOne = Run Grp

# &fras "input.txt"
# partition box ≠ over @\n
# reduce ++1 where each PartOne
