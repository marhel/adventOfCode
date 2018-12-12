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

function next_state((diff, state), notes)
    flower(pots) = get(notes, pots, '.')
    parts = IterTools.partition("....$state....", 5, 1)
    new_state = join([flower(pots) for pots in parts])
    new_lstate = lstrip(new_state, '.')
    ldiff = length(new_state) - length(new_lstate) - 2
    diff + ldiff, rstrip(new_lstate, '.')
end

function gen(initial, n, notes)
    evolve(state) = next_state(state, notes)
    generations = IterTools.iterated(evolve, (0, initial))
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

(o, eg20) = gen(exinitial, 20, exnotes)
@test "#....##....#####...#######....#.#..##" == eg20
@test -2 == o

function sum_alive((offset, state))
    indexed_gen = zip(state, (0:length(state)) .+ offset)
    alive = filter(v -> first(v) == '#', collect(indexed_gen))
    sum(map(last, alive))
end

@test 325 == sum_alive(gen(exinitial, 20, exnotes))

part1() = sum_alive(gen(initial, 20, notes))

# gen(initial, 500, notes)
(  473, "##...##...##...##...##...##...##...##...##...##...##...##...##...##...##...##...##...##...##...##...##...##...##...##...##...##")

# gen(initial, 5000, notes)
( 4973, "##...##...##...##...##...##...##...##...##...##...##...##...##...##...##...##...##...##...##...##...##...##...##...##...##...##")

# gen(initial, 50000, notes)
(49973, "##...##...##...##...##...##...##...##...##...##...##...##...##...##...##...##...##...##...##...##...##...##...##...##...##...##")
# gen(initial, 50000000000, notes)
gen5b = (49999999973, "##...##...##...##...##...##...##...##...##...##...##...##...##...##...##...##...##...##...##...##...##...##...##...##...##...##")

part2() = sum_alive(gen5b)
