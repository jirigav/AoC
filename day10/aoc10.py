def load_data(filename):
    data = []
    with open(filename, "r") as file:
        for line in file:
            data.append(int(line))
    return data

def part1(data):
    diff3 = 0
    diff1 = 0
    for i in range(len(data) - 1):
        if data[i + 1] - data[i] == 1:
            diff1 += 1
        if data[i + 1] - data[i] == 3:
            diff3 += 1
    return diff3*diff1

def part2(data):
    sequences = []
    seq = 1
    routes = [0, 1, 2, 4, 7, 13]
    for i in range(len(data) - 1):
        if data[i + 1] - data[i] == 1:
            seq += 1
        if data[i + 1] - data[i] == 3:
            if seq > 1:
                sequences.append(seq)
            seq = 1
    result = 1
    for s in sequences:
        result *= routes[s - 1]
    return result

if __name__ == '__main__':
    data = load_data("input")
    data.append(max(data) + 3)
    data.append(0)
    data.sort()
    print("part1:", part1(data))
    print("part2:", part2(data))    
