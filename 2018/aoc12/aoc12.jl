using Test

initial = "###.......##....#.#.#..###.##..##.....#....#.#.....##.###...###.#...###.###.#.###...#.####.##.#....#"

snotes = Dict("....." => '.',
"#..##" => '.',
"..###" => '#',
"..#.#" => '#',
".#.#." => '.',
"####." => '.',
"##.##" => '#',
"#...." => '.',
"#...#" => '.',
"...##" => '.',
"##..#" => '.',
".###." => '#',
"#####" => '#',
"#.#.." => '#',
".##.." => '#',
".#.##" => '.',
"...#." => '#',
"#.##." => '#',
"..#.." => '#',
"##..." => '#',
"....#" => '.',
"###.#" => '#',
"#..#." => '#',
"#.###" => '#',
"##.#." => '.',
"###.." => '#',
".####" => '.',
".#..." => '#',
"..##." => '.',
".##.#" => '.',
"#.#.#" => '#',
".#..#" => '.')


adapt_notes(snotes) = Dict(map(kv -> NTuple{5,Char}(collect(first(kv))) => last(kv), collect(snotes)))
notes = adapt_notes(snotes)

function next_state(state, notes)
    flower(pots) = get(notes, pots, '.')
    parts = IterTools.partition("....$state....", 5, 1)
    join([flower(pots) for pots in parts])
end

function gen(initial, n, notes)
    evolve(state) = next_state(state, notes)
    generations = IterTools.iterated(evolve, initial)
    IterTools.take(generations, n+1) |>
        collect |>
        last    
end

exsnotes = Dict("...##" => '#',
"..#.." => '#',
".#..." => '#',
".#.#." => '#',
".#.##" => '#',
".##.." => '#',
".####" => '#',
"#.#.#" => '#',
"#.###" => '#',
"##.#." => '#',
"##.##" => '#',
"###.." => '#',
"###.#" => '#',
"####." => '#')

exnotes = adapt_notes(exsnotes)
exinitial = "#..#.#..##......###...###"

eg20 = gen(exinitial, 20, exnotes)
@test occursin(".#....##....#####...#######....#.#..##.", eg20)

function sum_alive(initial, notes)
    g = 20
    eg20 = gen(initial, g, notes)
    indexed_gen = zip(eg20, (0:length(eg20)) .- 2g)
    alive = filter(v -> first(v) == '#', collect(indexed_gen))
    sum(map(last, alive))
end

@test 325 == sum_alive(exinitial, exnotes)

part1() = sum_alive(initial, notes)
