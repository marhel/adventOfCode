using Test

hundred(v) = convert(Int, floor(v / 100)) % 10
@test 8 == hundred(8842)
@test 4 == hundred(428)
@test 2 == hundred(284)
@test 0 == hundred(48)
@test 0 == hundred(4)

power(r, c, s) = (rack = (r + 10); hundred((rack * c + s) * rack) - 5)
@test 4 == power(3, 5, 8)
@test -5 == power(122, 79, 57)
@test 0 == power(217, 196, 39)
@test 4 == power(101, 153, 71)


ex18 = [ -2  -4   4   4   4;
         -4   4   4   4  -5;
          4   3   3   4  -4;
          1   1   2   4  -3;
         -1   0   2  -5  -2]
ex42 = [ -3   4   2   2   2;
         -4   4   3   3   4;
         -5   3   3   4  -4;
          4   3   3   4  -3;
          3   3   3  -5  -1]
gridview(grid, (r,c), (w,h)=(3,3)) = view(grid, (1:w) .+ (c - 1), (1:h) .+ (r - 1))
gridsum(grid, pos, dim=(3,3)) = sum(gridview(grid, pos, dim))
@test 29 == gridsum(ex18, (2,2))
@test 30 == gridsum(ex42, (2,2))

grid(s) = [power(r,c,s) for c in 1:300, r in 1:300]
function max_power(s, dim = 3:3)
    sgrid = grid(s)
    gsums = [[gridsum(sgrid, (r, c), (d, d)) => (r,c,d) for c in 1:(300-d), r in 1:(300-d)] for d in dim]
    maximum(Iterators.flatten(gsums))
end
@test 29 == max_power(18).first # total power of 29
@test 30 == max_power(42).first # total power of 30
@test (33, 45) == max_power(18).second # total power of 29
@test (21, 61) == max_power(42).second # total power of 30

part1() = max_power(8444)
part2() = max_power(8444, 1:300)
