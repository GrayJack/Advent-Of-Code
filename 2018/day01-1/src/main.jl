input = open("../input/input.txt")
freqs = read(input, String)

total = 0

for f in split(freqs)
    num = parse.(Int, f)
    global total += num
end

println(total)

close(input)
