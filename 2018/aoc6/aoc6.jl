struct Point
    x
    y
end

distance(a, b) = abs(a.x-b.x) + abs(a.y-b.y)

# if p has none above or below or left or right its area is infinite
function above(a, b)
    dx = a.x - b.x
    dy = a.y - b.y
    println("(", a, ", ", b, ")", ": ", dy, " ", dx)
    r = []
    if dy < 0 && abs(dx) < abs(dy)
        push!(r, :above)
    end
    if dy > 0 && abs(dx) < abs(dy)
        push!(r, :below)
    end
    if dx < 0 && abs(dx) > abs(dy)
        push!(r, :left)
    end
    if dx > 0 && abs(dx) > abs(dy)
        push!(r, :right)
    end
    if length(r) > 0
        pop!(r)
    else
        :none
    end
end

above(Point(14, 15), Point(10,10))
above(Point(15, 15), Point(10,10))
above(Point(15, 14), Point(10,10))
#   000000000011111111112222222222
#   012345678901234567890123456789
# 00         x3    x2
# 01
# 02               x1
# 03
# 04
# 05
# 06
# 07
# 08
# 09
