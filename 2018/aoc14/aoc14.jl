using Test

function next_state(elves, state)
    scores = sum_score(state[elves])
    pos = next_pos(elves, state[elves], length(state) + length(scores))
    pos, append!(state, scores)
end

function next_state_partial(elves, state)
    scores = sum_score(state[elves])
    pos = next_pos(elves, state[elves], length(state) + length(scores))
    pos, append!(state, scores), scores
end

next_pos(elves, score, len) = (elves + score) .% len .+ 1
sum_score(score) = map(v -> parse(Int, v), collect("$(sum(score))"))
build_score(pos, state) = parse(Int, join(Iterators.take(Iterators.drop(state, pos), 10)))

function score_after(n)
    state = [3, 7]
    elves = [1, 2]
    while length(state) < 10 + n
        # println(state)
        elves, state = next_state(elves, state)
    end
    build_score(n, state)
end

@test [1, 2] == next_pos([1, 2], [3, 7], 2)
@test [5, 4] == next_pos([1, 2], [3, 7], 6)
@test [7, 5] == next_pos([5, 4], [1, 0], 7)
@test [1, 7] == next_pos([7, 5], [1, 1], 8)
@test [6]    == sum_score([1, 5])
@test [1, 2] == sum_score([7, 5])
@test [1, 7] == sum_score([9, 8])
@test ([1, 2], [3, 7, 1, 0]) == next_state([1, 2], [3, 7])
@test ([5, 4], [3, 7, 1, 0, 1, 0]) == next_state([1, 2], [3, 7, 1, 0])
@test 5158916779 == build_score(9, [3,7,1,0,1,0,1,2,4,5,1,5,8,9,1,6,7,7,9,2])

function score_before(recipestr::String)
    recipe = map(v -> parse(Int, v), collect(recipestr))
    state = [3, 7]
    elves = [1, 2]
    rlen = length(recipe)
    at = 0
    state_len = 0
    count = 0
    rolling_tail = Int[]
    found = false
    score = []
    total = length(score)
    while !found
        count += 1
        if count % 100000 == 0
            print('*')
        end
        # println(state)
        elves, state, score = next_state_partial(elves, state)
        for s in score
            if total >= rlen
                #println(rolling_tail, ": ", total)
                rolling_tail = collect(Iterators.drop(rolling_tail, 1))
            end
            total += 1
            append!(rolling_tail, s)
            # println("rt: ", rolling_tail, " => ", recipe, " ???", recipestr, ", ", state)
            if recipe == rolling_tail
                found = true
                break
            end
        end
    end
    println(join(rolling_tail))
    total - rlen + 2
end

# (3)[7]
# (3)[7] 1  0 
#  3  7  1 [0](1) 0 
#  3  7  1  0 [1] 0 (1)
# (3) 7  1  0  1  0 [1] 2 
#  3  7  1  0 (1) 0  1  2 [4]
#  3  7  1 [0] 1  0 (1) 2  4  5 
#  3  7  1  0 [1] 0  1  2 (4) 5  1 
#  3 (7) 1  0  1  0 [1] 2  4  5  1  5 
#  3  7  1  0  1  0  1  2 [4](5) 1  5  8 
#  3 (7) 1  0  1  0  1  2  4  5  1  5  8 [9]
#  3  7  1  0  1  0  1 [2] 4 (5) 1  5  8  9  1  6 
#  3  7  1  0  1  0  1  2  4  5 [1] 5  8  9  1 (6) 7 
#  3  7  1  0 (1) 0  1  2  4  5  1  5 [8] 9  1  6  7  7 
#  3  7 [1] 0  1  0 (1) 2  4  5  1  5  8  9  1  6  7  7  9 
#  3  7  1  0 [1] 0  1  2 (4) 5  1  5  8  9  1  6  7  7  9  2 

@test 5158916779 == score_after(9)
@test 124515891 == score_after(5)
@test 9251071085 == score_after(18)
@test 5941429882 == score_after(2018)
@test 9 == score_before("51589")
@test 5 == score_before("01245")
@test 18 == score_before("92510")
@test 2018 == score_before("59414")

part1() = score_after(846601)
part2() = score_before("846601")