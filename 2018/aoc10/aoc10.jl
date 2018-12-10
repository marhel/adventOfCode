struct Coord
    x
    y
end
Base.:(+)(a::Coord, b::Coord) = Coord(a.x + b.x, a.y + b.y)
Base.:(*)(a::Coord, f::Int) = Coord(a.x * f, a.y * f)

function read(fname)
    starRx = r"position=<(?P<position>[- ]*\d+, [- ]*\d+)> velocity=<(?P<velocity>[- ]*\d+, [- ]*\d+)>"
    data = Dict{Char,Set{Char}}()
    return Channel() do c
        open(fname) do f
            for line in eachline(f)
                coords = match(starRx, line)
                # println(line, coords)
                toCoord(s) = Coord(map(v -> parse(Int, v), split(s, ", "))...)
                pos = toCoord(coords["position"])
                vel = toCoord(coords["velocity"])
                push!(c, (pos, vel))
            end
        end
    end
end

sstars = [v for v in read("simple-input.txt")]
stars = [v for v in read("input.txt")]
timestep(t, s) = Channel() do c
    for (pos,vel) in s
        push!(c, pos + vel * t)
    end
end

using StatsBase

function analyze(tmin, tmax,s,f=1)
    for t in tmin:tmax
        m = [p for p in timestep(t*f,s)]
        x = map(v -> v.x, m)
        y = map(v -> v.y, m)
        xcounts = StatsBase.countmap(x)
        ycounts = StatsBase.countmap(y)
        xm = Statistics.mean(values(xcounts))
        ym = Statistics.mean(values(ycounts))
        if xm > 5 && ym > 10
            println("$t: xmean $(xm) ($(length(xcounts))), ymean $(ym) ($(length(ycounts)))")
            # break
        end
    end
end

using Plots

function visa(t,s)
    m = [p for p in timestep(t,s)]
    x = map(v -> v.x, m)
    y = map(v -> -v.y, m)
    Plots.scatter(x, y)
end

part1() = visa(10418, stars)
part2() = 10418 # ;-)
