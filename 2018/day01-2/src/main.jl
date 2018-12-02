input = open("../input/input.txt")
freqs = read(input, String)

total = 0
seen = Set([0])
twice = -1

while twice <= -1
    for f in split(freqs)
        num = parse.(Int, f)
        global total += num

        if total in seen
            global twice = total
            break
        end

        push!(seen, total)
    end
end

println(twice)

close(input)
