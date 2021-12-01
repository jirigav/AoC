def load_input(filename):
    data = []
    with open(filename, "r") as file:
        for line in file:
            line = line[:-1].split(" ")
            data.append([line[0], int(line[1]), False])
    return data

def part1(data):
    i = 0
    acc = 0
    while 1:
        if data[i][2] == True:
            return acc

        data[i][2] = True

        if data[i][0] == "acc":
            acc += data[i][1]

        if data[i][0] == "jmp":
            i += data[i][1]
        else:
            i += 1


def part2(data):
    i = 0
    acc = 0
    changed = []
    round_changed = False
    while 1:
        if i == len(data):
            return acc

        if data[i][2] == True:
            i = 0
            round_changed = False
            acc = 0
            for d in data:
                d[2] = False

        data[i][2] = True

        instruction = data[i][0]

        if i not in changed and not round_changed:
            changed.append(i)
            round_changed = True
            instruction = "jmp" if data[i][0] == "nop" else "nop"

        if instruction == "acc":
            acc += data[i][1]

        if instruction == "jmp":
            i += data[i][1]
        else:
            i += 1        


if __name__ == '__main__':
    data = load_input("input")
    print("part1:", part1(data))
    print("part2:", part2(data))
 