function build_depgraph(fname)
    stepRx = r"Step (?P<first>\w+) must be finished before step (?P<later>\w+) can begin."
    data = Dict{Char,Set{Char}}()
    open(fname) do f
        for line in eachline(f)
            step = match(stepRx, line)
            # one based indexing...
            firstChar = step["first"][1]
            laterChar = step["later"][1]
            if !haskey(data, firstChar)
                data[firstChar] = Set{Char}([])
            end
            deps = get(data, laterChar, Set{Char}([]))
            data[laterChar] = push!(deps, firstChar)
        end
    end
    data
end

unblocked(tup) = length(last(tup)) == 0
ready(data) = filter(unblocked, data) |>
              keys |>
              collect |> 
              sort

without(key, data) = Dict(map(tup -> first(tup) => setdiff(last(tup), Set([key])), collect(data)))

function remove(keys, data)
    red = reduce((x, y) -> without(first(y),x), keys, init=data)
    for key in keys
        delete!(red, key)
    end
    red
end

function part1(data)
    ans = []
    while length(data) > 0
        char = first(ready(data))
        # print(char, length(data))
        push!(ans, char)
        data = remove([char], data)
    end
    join(ans)
end

do_part1() = part1(build_depgraph("input.txt"))

function part2(data, num_workers, delay=0)
    ans = []
    second = 0
    workers = repeat([0], num_workers)
    items = repeat(['?'], num_workers)
    delays = []
    completed = Char[]
    inprogress = Char[]
    ready_ch = ready(data)
    while length(data) > 0
        nonidle = filter(d -> d > 0, workers)
        if length(nonidle) > 0
            second = minimum(nonidle)
        end
        completed = Char[]
        for i in eachindex(workers)
            if workers[i] <= second && items[i] != '?'
                push!(completed, items[i])
                items[i] = '?'
                workers[i] = 0
            end
        end
        if length(completed) > 0
            data = remove(completed, data)
            # remove items already in progress
            ready_ch = sort(collect(setdiff(Set(ready(data)), Set(items))))
            println(second, ": completed: ", completed, " ready_ch: ", ready_ch)
        end
        available = filter(i -> workers[i] <= second, eachindex(workers))
        assigned = collect(Iterators.take(ready_ch, length(available)))
        println(second, ": available: ", available, ", ready_ch: ", ready_ch, ", assigned: ", assigned)
        delays = map(c -> c - '@' + delay + second, assigned)
        for i in eachindex(delays)
            wi = available[i]
            workers[wi] = delays[i]
            items[wi] = assigned[i]
        end
        println(second, ": workers: ", workers, ", items: ", items, ", datal: ", length(data))
    end
    second
end

sdata = build_depgraph("simple-input.txt")
data = build_depgraph("input.txt")
part2(sdata, 2)
println("\n***\n")
# part2(data, 5, 60)

# 1078 was too high
# 427 is too low