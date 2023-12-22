Matching ← /+ ∊ ⊃↙ ↘ : ↘ 1 ⋕ regex "\\d+" :
.&fras "input.txt"

# Part 1
Points ← ⊂ 0 ⁿ :2 ⇡ 10
Score ← ⊏ : Points Matching
&s /+ ∵Score 10 ⊜□ ≠ @\n .

# Part 2
⧻ . ∵Matching 10 ⊜□ ≠ @\n .
N ←
Dist ← +1 ⊙(+ ↻ ¯ +1 ⊙(⬚0 ↙ N ↯ : ⊙⊡ ⊡)) .. ⊙⊙⊙,,,
/+ ;:; ⍥Dist N 0 ↯ N 1

# Matching = reduce add member fork take drop flip drop 1 parse regex "\\d+" flip
# duplicate &fras "input.txt"

# Part 1
# Points = join 0 power flip 2 range 10
# Score = select : Points Matching
# show reduce add each Score 5 box != @\n duplicate

# Part 2
# length duplicate each Matching 10 partition box != @\n duplicate
# N = 
# Dist = add 1 dip (add rotate neg +1 dip (fill 0 take N reshape flip dip pick pick)) .. dipdipdip,,,
# reshape N 1

# reduce add pop flip pop repeat Dist N 0

# &fras "input.txt"
# reduce add each Score 10 partition box != @\n duplicate

---
assert ⊃⋅∘≍ 8 Score 5 $ Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
assert ⊃⋅∘≍ 2 Score 5 $ Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
assert ⊃⋅∘≍ 2 Score 5 $ Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
assert ⊃⋅∘≍ 1 Score 5 $ Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
assert ⊃⋅∘≍ 0 Score 5 $ Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
assert ⊃⋅∘≍ 0 Score 5 $ Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11

---