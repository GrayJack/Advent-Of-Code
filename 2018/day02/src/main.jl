function first_part(input)
    twice, thrice = 0, 0

    for line in String.(split(input))
        freqs = Array{Int8}(undef, 256)

        for i in line
            freqs[Int8(i)] += 1
        end

        if any(x -> x == 2, freqs)
            twice += 1
        end
        if any(x -> x == 3, freqs)
            thrice += 1
        end
    end

    println(twice * thrice)
end

function second_part(args)
    ids = collect(String.(split(input)))

    for i in ids, j in ids
        # println(i," ", j)
        common = find_common_letters(i, j)
        if !isempty(common)
            println(common)
            break
        end
    end

end

function find_common_letters(id1, id2)
    if id1 == id2 || length(id1) != length(id2)
        return []
    end

    found_once = false
    ids_tuple = zip(id1, id2)
    for (c1, c2) in ids_tuple
        if c1 != c2
            if found_once
                return []
            end
            found_once = true
        end
    end

    return collect(
        map((c, _) -> c,
        filter((c1, c2) -> c1 == c2,
        ids_tuple)))
end


file = open("../input/input.txt")
input = read(file, String)

first_part(input)
second_part(input)

close(file)
