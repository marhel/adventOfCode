Points ← ⊂ 0 ⁿ :2 ⇡ 10
Score ← ⊏ : Points /+ ∊ ⊃↙ ↘ : ↘ 1 ⋕ regex "\\d+" :

&fras "input.txt"
/+ ∵Score 10 ⊜□ ≠ @\n .

# Points = join 0 power flip 2 range 10
# Score = select : Points reduce add member fork take drop flip drop 1 parse regex "\\d+" flip

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