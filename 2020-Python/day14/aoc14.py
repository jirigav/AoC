def load_data(input):
    with open(input, "r") as file:
        return file.readlines()

def sumation(bitmap, index):
    sumation = 0
    for index in bitmap:
        for i in range(len(bitmap[index])):
            sumation += bitmap[index][i]*2**i
    return sumation

def possible_values(mask):
    p = 2**(len(mask) - 1)
    value = 0
    for i in range(len(mask)):
        if mask[i] == "1":
            value += p
        elif mask[i] == "X":

            prev_vals = possible_values(mask[(i + 1):])

            curr_vals = []
            for prev_val in prev_vals:
                curr_vals.append(prev_val + value + p)
                curr_vals.append(prev_val + value)
            return curr_vals
        p //= 2
    return [value]

def mask_i(mask, index):
    p = 34359738368
    i = 35
    while i >= 0:
        if p <= index:
            index -= p
            if mask[i] == "0":
                mask[i] = "1"
        i -= 1
        p //= 2
    return mask

def part1(data):
    bitmap = {}
    mask = ""
    for line in data:
        if "mask" == line[0:4]:
            mask = line[7:]
            mask = [char for char in mask[:-1]]
            mask = mask[::-1]

        if "mem" == line[0:3]:
            line = line.split("]")
            index = int(line[0][4:])
            value = int(line[1][3:])
            
            mem = [0]*36
            p = 34359738368
            i = 35
            while i >= 0:
                if value >= p:
                    value -= p
                    if mask[i] == "X" or mask[i] == "1":
                        mem[i] = 1
                    else:
                        mem[i] = 0
                else:
                    if mask[i] == "1":
                        mem[i] = 1
                    else:
                        mem[i] = 0
                i -= 1
                p //= 2
            bitmap[index] = mem

    return sumation(bitmap, index)

def part2(data):
    bitmap = {}
    mask = ""
    for line in data:
        if "mask" == line[0:4]:
            mask = line[7:]
            mask = [char for char in mask[:-1]]
            mask = mask[::-1]

        if "mem" == line[0:3]:
            line = line.split("]")
            index = int(line[0][4:])
            value = int(line[1][3:])
            maski = mask_i(mask.copy(), index)
            indices = possible_values(maski[::-1])

            
            mem = [0]*36
            p = 34359738368
            i = 35
        
            while i >= 0:
                if value >= p:
                    value -= p
                    mem[i] = 1
                i -= 1
                p //= 2

            for index in indices:
                bitmap[index] = mem

    return sumation(bitmap, index)

if __name__ == '__main__':
    data = load_data("input")
    print("part1:", part1(data))
    print("part2:", part2(data))
