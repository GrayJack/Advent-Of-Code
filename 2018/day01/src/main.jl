function first_part(freqs)
    total = 0

    for f in split(freqs)
        num = parse.(Int, f)
        total += num
    end

    println(total)
end

function second_part(freqs)
    total = 0
    seen = Set([0])
    twice = -1

    while twice <= -1
        for f in split(freqs)
            num = parse.(Int, f)
            total += num

            if total in seen
                twice = total
                break
            end

            push!(seen, total)
        end
    end

    println(twice)
end

file = open("../input/input.txt")
freqs = read(file, String)

first_part(freqs)
second_part(freqs)

close(file)
