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


struct Node
    start
    stop
    meta
    children
end
value(n) = if length(n.children) == 0
    println("value of $(n.start) w/o children")
    sum(n.meta)
else
    println("value of $(n.start) w/children")
    sum(Int[value(n.children[m]) for m in n.meta if m in 1:length(n.children)])
end

function parseNodes(arr, i)
    childcount = arr[i]
    metacount = arr[i+1]
    println("node $childcount/$metacount at ", i)
    pos = i + 1
    metasum = 0
    children = Node[]
    for c in 1:childcount
        child = parseNodes(arr, pos+1)
        push!(children, child)
        pos = child.stop
    end
    meta = view(arr, (1:metacount) .+ pos)
    Node(i, pos + metacount, meta, children)
end

function part2(line)
    numstrs = split(line, ' ')
    numbers = map(v -> parse(Int, v), numstrs)
    root = parseNodes(numbers, 1)
    println(root)
    println(value(root))
end
