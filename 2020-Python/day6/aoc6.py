
if __name__ == '__main__':
    sum_part1 = 0
    sum_part2 = 0
    with open("input", "r") as file:
        first_line = True
        answers_part1 = set()
        answers_part2 = set()
        for line in file:
            if line == "\n":
                sum_part1 += len(answers_part1)
                sum_part2 += len(answers_part2)
                first_line = True
            else:
                if first_line:
                    answers_part1 = set([char for char in line[:-1]])
                    answers_part2 = set([char for char in line[:-1]])
                    first_line = False
                else:
                    answers_part1 |= set([char for char in line[:-1]])
                    answers_part2 &= set([char for char in line[:-1]])
                
    print("part1:", sum_part1)
    print("part2:", sum_part2)
