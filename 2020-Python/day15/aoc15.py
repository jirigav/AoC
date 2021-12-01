def main(input, turns):
    last_call = {}
    for i in range(len(input)):
        last_call[input[i]] = i + 1

    turn = len(input) + 1
    curr_n = 0

    while turn < turns:
        next_n = 0
        if curr_n in last_call:
            next_n = turn - last_call[curr_n]

        last_call[curr_n] = turn

        curr_n = next_n
        turn += 1
    return curr_n

if __name__ == '__main__':
    input = [0, 12, 6, 13, 20, 1, 17]
    print("part1:", main(input, 2020))
    print("part2:", main(input, 30000000))
    