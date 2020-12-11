from copy import deepcopy

def load_data(filename):
    data = []
    with open(filename, "r") as file:
        for line in file:
            data.append([char for char in line[:-1]])
    return data

def count_occupied(data):
    count = 0
    for i in range(len(data)):
        for j in range(len(data[0])):
            if data[i][j] == "#":
                count += 1
    return count

def count_occupied_directions1(data, start):
    count = 0
    for ii in range(max(start[0] - 1, 0), min(start[0] + 2, len(data))):
        for jj in range(max(start[1] - 1, 0), min(start[1] + 2, len(data[0]))):
            if data[ii][jj] == "#":
                count += 1
    return count

def direction_occupied(data, direction, start):
    a = start[0] + direction[0]
    b = start[1] + direction[1]
    while 0 <= a < len(data) and 0 <= b < len(data[0]):
        if data[a][b] == "#":
            return True
        if data[a][b] == "L":
            return False
        a += direction[0]
        b += direction[1]
    return False

def count_occupied_directions2(data, start):
    count = 0
    for i in range(-1, 2):
        for j in range(-1, 2):
            if i == 0 and j == 0:
                continue
            if direction_occupied(data, (i, j), start):
                count += 1
    return count

def main(data, part):
    no_change = False
    new_data = [[0]*len(data[0]) for _ in range(len(data))]
    while not no_change:
        no_change = True

        for i in range(len(data)):
            for j in range(len(data[0])):

                if data[i][j] == ".":
                    new_data[i][j] == "."
                if part == 1:
                    occup = count_occupied_directions1(data, (i, j))
                else:
                    occup = count_occupied_directions2(data, (i, j))

                if data[i][j] == "L":
                    if not occup:
                        new_data[i][j] = "#"
                        no_change = False
                    else:
                        new_data[i][j] = "L"

                elif data[i][j] == "#":
                    if occup > 4:
                        new_data[i][j] = "L"
                        no_change = False
                    else:
                        new_data[i][j] = "#"
        data, new_data = new_data, data

    return count_occupied(data)

if __name__ == '__main__':
    data = load_data("input")
    print("part1:", main(deepcopy(data), 1))
    print("part2:", main(data, 2))
    