struct Claim
    id::UInt32
    x_inches::UInt32
    y_inches::UInt32
    x_size::UInt32
    y_size::UInt32
end

struct IterPoints
    claim::Claim
    px::UInt32
    py::UInt32
end

file = open("../input/input.txt")
input = read(file, String)

first_part(input)
second_part(input)

close(file)
