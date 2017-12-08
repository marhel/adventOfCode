cat input7.1.txt | grep - | cut -f2 -d'>' | tr ', ' '\n' | sort | uniq > non-leaves.txt
cat input7.1.txt | grep - | cut -f1 -d' ' | sort | uniq > candidates.txt
comm -3 candidates.txt non-leaves.txt | grep  -E "^\w"
