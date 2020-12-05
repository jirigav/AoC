
def load_plan(file_name):
    plan = []
    with open(file_name, "r") as file:
        for line in file:
            line = line[:-1]
            plan.append([char for char in line])
    return plan

def tobogan(changex, changey, plan):
    count = 0
    x = 0
    y = 0
    while y < len(plan):
        if plan[y][x%len(plan[0])] =="#":
            count += 1
        x += changex
        y += changey
    return count


if __name__ == '__main__':
    plan = load_plan("input")
    print("part1:", tobogan(3, 1, plan))

    result2 = 1
    for (changex, changey) in [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]:
        result2 *= tobogan(changex, changey, plan)

    print("part2:", result2)
