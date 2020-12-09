def load_data(filename):
    data = []
    with open(filename, "r") as file:
        for line in file:
            data.append(int(line))
    return data

def part1(data):
    preamble = []
    for i in range(25):
        preamble.append(data[i])

    for i in range(25, len(data)):
        sum_of_two = False
        for a in preamble:
            for b in preamble:
                if a + b == data[i]:
                    sum_of_two = True
        if not sum_of_two:
            return data[i]
        preamble = preamble[1:]
        preamble.append(data[i])

def part2(data):
    result1 = part1(data)
    preamble = [data[0]]
    i = 1
    while i < len(data):
        if sum(preamble) == result1:
            return min(preamble) + max(preamble)
            break

        if sum(preamble) < result1:
            preamble.append(data[i])
            i += 1

        if sum(preamble) > result1:
            preamble = preamble[1:]

if __name__ == '__main__':
    data = load_data("input")
    print("part1:", part1(data))
    print("part2:", part2(data))
