import copy

def load_data(input):
    with open(input, "r") as file:
        return file.readlines()

def format_data(data, rounds):
    d = {"." : 0, "#" : 1}
    data3d = [[[0 for _ in range(len(data) + 2*rounds)] for _ in range(len(data) + 2* rounds)] for _ in range(2*rounds + 1)]
    for i in range(len(data)):
        for j in range(len(data)):
            data3d[rounds][i + rounds][j + rounds] = d[data[i][j]]
    return data3d

def sum_neighbors(data, coordinates):
    sumation = 0
    for k in range(-1, 2):
        for i in range(-1, 2):
            for j in range(-1, 2):
                if k == i == j == 0:
                    continue
                ck = k + coordinates[0]
                ci = i + coordinates[1]
                cj = j + coordinates[2]
                if 0 <= ck < len(data) and 0 <= ci < len(data[0]) and 0 <= cj < len(data[0][0]):
                    sumation += data[ck][ci][cj]
    return sumation
def sum_neighbors4d(data, coordinates):
    sumation = 0
    for m in range(-1, 2):
        for k in range(-1, 2):
            for i in range(-1, 2):
                for j in range(-1, 2):
                    if k == i == j == m == 0:
                        continue
                    cm = m + coordinates[0]
                    ck = k + coordinates[1]
                    ci = i + coordinates[2]
                    cj = j + coordinates[3]
                    if 0 <= cm < len(data) and 0 <= ck < len(data[0]) and 0 <= ci < len(data[0][0]) and 0 <= cj < len(data[0][0][0]):
                        sumation += data[cm][ck][ci][cj]
    return sumation


def part1(data):
    rounds = 6
    data3d = format_data(data, rounds)
    
    data3d_new = copy.deepcopy(data3d)
    for c in range(rounds):
        for k in range(2*rounds + 1):
            for i in range(len(data) + 2*rounds):
                for j in range(len(data) + 2*rounds):
                    sumation = sum_neighbors(data3d, (k, i, j))

                    if sumation == 3:
                        if i == 1:
                            print(8888)
                        data3d_new[k][i][j] = 1
                    elif sumation == 2 and data3d[k][i][j]:
                        data3d_new[k][i][j] = 1
                    else:
                        data3d_new[k][i][j] = 0
        data3d, data3d_new = data3d_new, data3d

    return sum(sum(sum(data3d, []),[]))


def part2(data):
    rounds = 6
    d = {"." : 0, "#" : 1}
    data4d = [[[[0 for _ in range(len(data) + 2*rounds)] for _ in range(len(data) + 2* rounds)] for _ in range(2*rounds + 1)] for _ in range(2*rounds + 1)]
    for i in range(len(data)):
        for j in range(len(data)):
            data4d[rounds][rounds][i + rounds][j + rounds] = d[data[i][j]]


    data4d_new = copy.deepcopy(data4d)
    for c in range(rounds):
        for m in range(2*rounds + 1):
            for k in range(2*rounds + 1):
                for i in range(len(data) + 2*rounds):
                    for j in range(len(data) + 2*rounds):
                        sumation = sum_neighbors4d(data4d, (m, k, i, j))
                        if sumation == 3:
                            data4d_new[m][k][i][j] = 1
                        elif sumation == 2 and data4d[m][k][i][j]:
                            data4d_new[m][k][i][j] = 1
                        else:
                            data4d_new[m][k][i][j] = 0
        data4d, data4d_new = data4d_new, data4d

    return sum(sum(sum(sum(data4d, []), []),[]))

if __name__ == '__main__':
    data = load_data("input")
    print("part1:", part1(data))
    print("part2:", part2(data))
    

    