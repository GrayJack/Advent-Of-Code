#!/bin/env julia

function part1(elems)
    [(e ÷ 3 - 2) for e in elems] |> sum
end

function helper(elem)
    elem = (elem ÷ 3) - 2
    ans = elem
    while ans >= 0
        ans = (ans ÷ 3) - 2
        if ans > 0
            elem += ans;
        end # if
    end
    elem
end # function

function part2(elems)
    [helper(e) for e in elems] |> sum
end

function main()
    if isempty(ARGS)
        println(stderr, "Usage:\n\tjulia main.jl <INPUT_FILE>\nOR\n\tmain.jl <INPUT_FILE>")
        exit(1)
    else
        elems = [parse(Int64, e) for e in read(ARGS[1], String) |> split]
    end

    println("Part 1: $(part1(elems))")
    println("Part 1: $(part2(elems))")
end

main()
