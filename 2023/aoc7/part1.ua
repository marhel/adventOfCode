Cards ← {@A @K @Q @J @T @9 @8 @7 @6 @5 @4 @3 @2}
ParseBy! ← ⊜(^1) ≠ @\n .
Wild ← ▽ ≠ @J .
Sub ← ⊡ ⍉ : ⊟ : "ZZZZZ" : ⊟ :⇡ 5 = , @J
Class!! ← (
  : ⊝ . . ^1
  ⊏ ⍖ °⊚ ⊛
  ⊛ ⊏ ⍏ ⊗ ,
  ^1
)
Score!!! ← (
  : ⊗ : Cards ^1 .
  ≡Class!!^1 ^1
  ⊏ ⍖ ? ≡⊂
  /+ × +1 ⇡ ⧻ .
)

&fras $ input.txt
ParseBy!(⊜□ ≠ @ .)
: ⋕ ≡(: °□ °⊟)
⊃Score!!!∘ ∘ ∘ Score!!!(⊏ ⍏ . ⬚0 ↙ 5) Wild (≡Sub)

# Cards = {@A @K @Q @J @T @9 @8 @7 @6 @5 @4 @3 @2}
# ParseBy! = partition (^1) != @\n duplicate
# Wild = keep != @J duplicate
# Sub ← pick transpose flip couple flip "ZZZZZ" flip couple flip⇡ 5 = over @J
# Class!! = (
#   flip deduplicate duplicate duplicate ^1
#   select fall un where classify 
#   classify select rise indexof over 
#   ^1
# )
# Score!!! = (
#   flip indexof flip Cards ^1 duplicate
#   rows Class!!^1 ^1
#   select fall ? rows join
#   /+ * +1 range length duplicate
# )

# &fras $ input.txt
# ParseBy!(partition box != @ duplicate)
# flip parse rows (flip unbox uncouple)
# fork Score!!!id id id Score!!!(select rise . fill 0 take 5) Wild (rows Sub)