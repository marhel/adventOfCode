using Test
using .Password

@testset "valid password" begin
@test !Password.valid(223450) # does not meet these criteria (decreasing pair of digits 50).
@test !Password.valid(123789) # does not meet these criteria (no double).
@test Password.valid(111111)     # meets these criteria (double 11, never decreases).
@test Password.valid2(112233) # meets these criteria because the digits never decrease and all repeated digits are exactly two digits long.
@test !Password.valid2(123444) # no longer meets the criteria (the repeated 44 is part of a larger group of 444).
@test Password.valid2(111122) # meets the criteria (even though 1 is repeated more than twice, it still contains a double 22).

@test Password.runcount(iseven, [1,2,4,6,8,10,9,7,5,2,1]) == [(false, 1), (true, 5), (false, 3), (true, 1), (false, 1)]
end
