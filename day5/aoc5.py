
def load_ids(file_name):
    id_list = []
    with open(file_name, "r") as file:

        for line in file:
            line = line.replace("F", "0")
            line = line.replace("B", "1")
            line = line.replace("L", "0")
            line = line.replace("R", "1")

            power = 512
            id = 0
            for n in line[0:10]:
                id += int(n)*power
                power //= 2
            
            id_list.append(id)
    return id_list


if __name__ == '__main__':
    id_list = load_ids("input")
    id_list.sort()

    print("part1:", id_list[-1])

    for i in range(len(id_list) - 1):
        if int(id_list[i]) + 2 == (int(id_list[i + 1])):
            print("part2:", int(id_list[i]) + 1)
            break
        