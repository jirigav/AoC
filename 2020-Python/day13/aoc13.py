
def load_data(input):
    with open(input, "r") as file:
        lines = file.readlines()
        time = int(lines[0])
        ids = lines[1][:-1].split(",")
        return time, ids

def part1(time, ids):
    min_t = 2*time
    min_id = 0
    for id in ids:
        if id == "x":
            continue
        id = int(id)
        if time%id == 0:
            print(0)
            continue
        diff = (time//id)*id + id
        if diff < min_t:
            min_id = id
            min_t = diff
    return min_id*(min_t - time)

def modular_inverse(a, m):
    if m == 0:
        return (1, 0)

    (q, r) = a // m, a % m
    (k, l) = modular_inverse(m, r)
    return (l, k - q * l)

def cr(a, m):
    sumation = 0
    product = 1
    for i in m:
        product *= i
    for i in range(len(m)):
        p = product // m[i]
        inv, _ = modular_inverse(p, m[i])
        sumation += a[i] * inv * p
    return sumation % product

def part2(ids):
    a = []
    ids_pruned = []
    for i in range(len(ids)):
        if ids[i] != "x":
            a.append(int(ids[i]) - i)
            ids_pruned.append(int(ids[i]))
 
    return cr(a, ids_pruned)

if __name__ == '__main__':
    time, ids = load_data("input")
    print("part1:", part1(time, ids))
    print("part2:", part2(ids))
