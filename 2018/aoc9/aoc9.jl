module AOC

using LinkedLists

function sequence()
    marble_id = -1
    () -> marble_id += 1, (i) -> marble_id = i, () -> marble_id
end

# skip the list head (which is pointed to by the first and last node)
function clockwise(list, node)
    if node.next == list.node
        node.next.next
    else
        node.next
    end
end

# skip the list head (which is pointed to by the first and last node)
function counter_clockwise(list, node)
    if node.prev == list.node
        node.prev.prev
    else
        node.prev
    end
end

times(f, n) = reduce((ff,v) -> ff ∘ f, 1:n-1, init=f)

insert_after(list, node, v) = insert!(list, node.next, v)

mutable struct Score
    player
    score
end

function play(players, end_score)
    marble_id = 0

    # a doubly linked list
    ring = LinkedList{Int}()
    ring.node.data = -1

    append!(ring, marble_id)
    current = ring.node.next

    cc7 = times(node -> counter_clockwise(ring, node), 7)
    function add_marble(players, scoring)
        scoring.player = 1 + marble_id % players
        next = marble_id += 1
        score = 0
        if next % 23 > 0
            current = insert_after(ring, clockwise(ring, current), next)
        else
            remove = cc7(current)
            current = clockwise(ring, remove)
            removed = splice!(ring, remove)
            score = next + removed
        end
        scoring.score = score
        nothing
    end

    println(ring)
    scoreboard = repeat([0], players)
    counter = 0
    # preallocating the result for `add_marble` removes one allocation per iteration
    scoring = Score(0, 0)
    while true
        counter += 1
        add_marble(players, scoring)
        scoreboard[scoring.player] += scoring.score
        if end_score <= marble_id
            println("win $scoring $(maximum(scoreboard))")
            break
        end
        # println("$p: $score")
    end
    maximum(scoreboard), counter
end

end # module