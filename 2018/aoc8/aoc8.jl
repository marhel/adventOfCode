function with_file(fname, fn)
    open(fname) do f
        for line in eachline(f)
            fn(line)
        end
    end
end

function part1(line)
    numstrs = split(line, ' ')
    numbers = map(v -> parse(Int, v), numstrs)
    root, mds = node(numbers, 1)
    println(root, " ", mds)
end

function node(arr, i)
    children = arr[i]
    metadata = arr[i+1]
    println("node $children/$metadata at ", i)
    pos = i + 1
    metasum = 0
    for c in 1:children
        pos, childsum = node(arr, pos+1)
        metasum += childsum
    end
    metasum += sum(view(arr, (1:metadata) .+ pos))
    pos + metadata, metasum
end
