module Password

using Base.Iterators

# It is a six-digit number.
# The value is within the range given in your puzzle input.
# Two adjacent digits are the same (like 22 in 122345).
# Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679).
# Other than the range rule, the following are true:

function valid(password)
    digs = digits(password)
    pairs = zip(digs, drop(digs, 1))
    same = any(v -> v[1] == v[2], pairs)
    decreases = any(v -> v[1] < v[2], pairs)
    length(digs) == 6 && same && !decreases
end

function allValid()
    [candidate for candidate=128392:643281  if valid(candidate)]
end

runcount(keyFn, list::Array) = begin
  foldl(list; init = []) do repeats, v
    k = keyFn(v)
    l = length(repeats)
    if length(repeats) == 0
        push!(repeats, (k, 1))
    else
        (rk, rv) = last(repeats)
        if rk == k
            repeats[l] = (k, rv+1)
        else
            push!(repeats, (k, 1))
        end
    end
    repeats
  end
end

twice((k,r)) = r == 2

function valid2(password)
    digs = digits(password)
    rcs = runcount(identity, digs)
    hastwice = any(twice, rcs)
    valid(password) && hastwice
end

function allValid2()
    [candidate for candidate=allValid() if valid2(candidate)]
end

end
