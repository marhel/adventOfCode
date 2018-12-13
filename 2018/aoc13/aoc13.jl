strack_str2 = read("simple-input2.txt", String)
strack_str = read("simple-input.txt", String)
track_str = read("input.txt", String)

# in order to transpose a Char array
Base.adjoint(c::Char) = c
function build_track(track_str)
    rows = split(track_str, '\n')
    charrows = map(collect, rows)
    track = hcat(charrows...)'
    convert(Array{Char, 2}, track)
end

strack = build_track(strack_str)
strack2 = build_track(strack_str2)
track = build_track(track_str)

directions = Dict(
    'X' => CartesianIndex(0, 0),
    '<' => CartesianIndex(0, -1),
    '>' => CartesianIndex(0, 1),
    'v' => CartesianIndex(1, 0),
    '^' => CartesianIndex(-1, 0))
follow = Dict(
    '<' => Dict('/' => 'v', '-' => '<', '\\' => '^'),
    '>' => Dict('/' => '^', '-' => '>', '\\' => 'v'),
    'v' => Dict('/' => '<', '|' => 'v', '\\' => '>'),
    '^' => Dict('/' => '>', '|' => '^', '\\' => '<'))
beneath = Dict(
    '<' => '-',
    '>' => '-',
    'v' => '|',
    '^' => '|')
turns = Dict(
    '<' => Dict(0 => 'v', 1 => '<', 2 => '^'),
    '>' => Dict(0 => '^', 1 => '>', 2 => 'v'),
    'v' => Dict(0 => '>', 1 => 'v', 2 => '<'),
    '^' => Dict(0 => '<', 1 => '^', 2 => '>'))

struct Cart
    heading
    position
    intersection
    beneath
end

is_cart(c) = c in ['<', '>', 'v', '^']
to_cart((cart, position)) = Cart(cart, position, 0, beneath[cart])
function extract_carts(track_with_carts)
    carts = map(to_cart, filter(v -> is_cart(first(v)), collect(zip(track_with_carts, CartesianIndices(track_with_carts)))))
    track = copy(track_with_carts)
    for cart in carts
        track[cart.position] = cart.beneath
    end
    carts, track
end
intersection_turn(heading, intersection) = turns[heading][intersection % 3]
function move_cart(cart, track)
    new_pos = cart.position + directions[cart.heading]
    beneath = track[new_pos]
    intersection = cart.intersection
    if beneath == '+'
        new_heading = intersection_turn(cart.heading, cart.intersection)
        intersection += 1
    else
        new_heading = follow[cart.heading][beneath]
    end
    # track[new_pos] = new_heading
    # track[cart.position] = cart.beneath
    Cart(new_heading, new_pos, intersection, beneath)
end

function to_rc0(ci)
    # ci is a CartesianIndex(col, row) where col and row are also one-based
    ci[2]-1, ci[1]-1
end

function part1(track_with_carts)
    carts, track = extract_carts(track_with_carts)
    while length(carts) > 0
        for i in sortperm(carts, by=c -> c.position)
            carts[i] = move_cart(carts[i], track)
            others = setdiff(Set(carts), Set([carts[i]]))
            other_positions = Set(map(c -> c.position, collect(others)))
            if carts[i].position in other_positions
                return to_rc0(carts[i].position)
            end
        end
    end
end

function show_carts(carts, track)
    track_with_carts = copy(track)
    for cart in carts
        track_with_carts[cart.position] = cart.heading
    end
    show(stdout, "text/plain", track_with_carts)
    println()
end

function part2(track_with_carts)
    carts, track = extract_carts(track_with_carts)
    # show(stdout, "text/plain", track_with_carts)
    # println(carts)
    crashed = Int[]
    while length(carts) - length(crashed) > 1
        for i in sortperm(carts, by=c -> c.position)
            if i ∉ crashed
                # println("moving cart $i")
                carts[i] = move_cart(carts[i], track)
                for j in eachindex(carts)
                    if j ∉ crashed && i != j && carts[i].position == carts[j].position
                        # println("crashed: $i/$j ", carts[i], " <X> ", carts[j])
                        push!(crashed, i)
                        push!(crashed, j)
                    end
                end
            end
        end
    end
    remaining = filter(i -> i ∉ crashed, eachindex(carts))
    to_rc0(first(carts[remaining]).position)
end
