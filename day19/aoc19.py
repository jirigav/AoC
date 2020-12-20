
def load_data(input):
    with open(input, "r") as file:
        return file.readlines()

def parse_rules_words(data):
    w = False
    words = []
    rules = {}
    for line in data:
        if line == "\n":
            w = True
            continue

        if w:
            words.append(line[:-1])
        if not w:
            line = line[:-1]
            line = line.split(":")
            rules[line[0]] = [part.split(" ") for part in line[1][1:].split(" | ")]
    return rules, words

def language(rules, rule):
    results = set()
    for r in rules[rule]:
        terminals = set()
        for minal in r:
            if minal.isnumeric():
                new_terminals = set()
                aaa = language(rules, minal)
                if len(terminals) == 0:
                    terminals = aaa
                    continue
                for terminal in terminals:
                    for a in aaa:
                        new_terminals.add(terminal + a)
                terminals = new_terminals
                
            else:
                new_terminals = set()
                if len(terminals) == 0:
                    terminals = {minal}
                    continue
                for terminal in terminals:
                    new_terminals.add(terminal + minal)
                terminals = new_terminals


        results |= terminals
    l = set()
    for word in results:
        l.add(word.replace('"', ''))

    return l

def part1(rules, words):
    l = language(rules, "0")
    counter = 0
    for word in words:
        if word in l:
            counter += 1
    return counter

def w_in_l42iter(w, l42):
    b = False
    for w1 in l42:
        if len(w1) > len(w):
            continue
        else:
            if w[:len(w1)] == w1:
                if len(w1) == len(w):
                    return True
                b = b or w_in_l42iter(w[len(w1):], l42)
    return b

def w_in_l0(w, l31, l42):
    b = False
    for w1 in l42:
        for w2 in l31:
            if len(w) <= len(w1 + w2):
                continue
            else:
                if w1 == w[:len(w1)] and w2 == w[len(w) - len(w2):]:
                    b = b or w_in_l0(w[len(w1):len(w)-len(w2)], l31, l42) or w_in_l42iter(w[len(w1):len(w)-len(w2)], l42)
    return b

def part2(rules, words):
    l31 = language(rules, "31")
    l42 = language(rules, "42")
    count = 0
    for w in words:
        if w_in_l0(w, l31, l42):
            count += 1
    return count

if __name__ == '__main__':
    data = load_data("input")
    rules, words = parse_rules_words(data)

    print("part1:", part1(rules, words))
    print("part2:", part2(rules, words))
