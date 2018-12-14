using Test

function next_state(elves, state)
    scores = sum_score(state[elves])
    pos = next_pos(elves, state[elves], length(state) + length(scores))
    pos, append!(state, scores)
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
    recipe = collect(recipestr)
    state = [3, 7]
    elves = [1, 2]
    rlen = length(recipe)
    at = 0
    state_len = 0
    while length(state) < 1000000000
        # println(state)
        elves, state = next_state(elves, state)
        state_len = length(state)
        if (t1l = state_len - rlen) >= 0
            tail1 = join(Iterators.drop(state, t1l))
            # println(recipestr, " ==1 ", tail1, ": ", join(state))
            if recipestr == tail1
                at = 0
                break
            end
        end
        if (t2l = state_len - rlen - 1) >= 0
            tail2 = join(Iterators.take(Iterators.drop(state, t2l), rlen))
            # println(recipestr, " ==2 ", tail2, ": ", join(state))
            if recipestr == tail2
                at = 1
                break
            end
        end
    end
    println(join(state))
    state_len - at - rlen
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
# 9994 is too low