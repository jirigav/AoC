from random import randint

def load_bags1(filename):
    bags = {}
    with open(filename, "r") as file:
        for line in file:
            words = line[:-1].split(" ")
            
            outter = words[0] + " " + words[1]
            inner = set()
            for i in range(4, len(words), 4):
                if not words[i].isnumeric():
                    continue
                inner.add(words[i + 1] + " " + words[i + 2])
            if outter in bags:
                bags[outter] = bags[outter] | inner
            else:
                bags[outter] = inner
    return bags

def load_bags2(filename):
    bags = {}
    with open(filename, "r") as file:
        for line in file:
            words = line[:-1].split(" ")
            
            outter = words[0] + " " + words[1]
            inner = set()
            for i in range(4, len(words), 4):
                if not words[i].isnumeric():
                    continue
                inner.add((words[i + 1] + " " + words[i + 2], int(words[i])))
            if outter in bags:
                bags[outter] = bags[outter] | inner
            else:
                bags[outter] = inner
    return bags

def contains(bag_name, bags):
    c = 1
    for bag in bags[bag_name]:
        a = contains(bag[0], bags)
        c += bag[1] * a
    return c

def part1():
    bags = load_bags1("input")
    contain = set()
    for bag in bags:
        if "shiny gold" in bags[bag]:
            contain.add(bag)
    checked = set()
    while contain:
        new_contain = set()
        for bag_inn in contain:
            checked.add(bag_inn)
            for bag in bags:
                if bag_inn in bags[bag] and bag not in checked:
                    new_contain.add(bag)
        contain = new_contain
    return len(checked)

def part2():
    bags = load_bags2("input")
    return contains("shiny gold", bags) - 1


if __name__ == '__main__':
    print("part1:", part1())
    print("part2:", part2())
