def load_pws_and_limits(file_name):
    limits = []
    pws = []

    with open(file_name, "r") as file:
        for line in file:
            line = line.split("-")
            limitLow = int(line[0])

            line = line[1].split(" ")
            limitUp = int(line[0])

            letter = line[1][:-1]

            pw = line[-1][:-1]

            limits.append((limitLow, limitUp, letter))
            pws.append(pw)

    return (pws, limits)

def part1(pws, limits):
    count = 0
    for i in range(len(pws)):
        if limits[i][0] <= pws[i].count(limits[i][2]) <= limits[i][1]:
            count += 1
    return count

def part2(pws, limits):
    count = 0
    for i in range(len(pws)):
        if (pws[i][limits[i][0] - 1] == limits[i][2]) != (pws[i][limits[i][1] - 1] == limits[i][2]):
            count += 1
    return count

if __name__ == '__main__':
    pws, limits = load_pws_and_limits("input")

    print("part1", part1(pws, limits))

    print("part2", part2(pws, limits))
