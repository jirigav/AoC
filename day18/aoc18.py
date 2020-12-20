
def load_data(input):
    with open(input, "r") as file:
        return file.readlines()

def calculate(s):
    value = 0
    operator = "+"
    i = 0
    rvalue = 0
    while i < len(s):
        if s[i] == "(":
            counter = 1
            i += 1
            substr = ""
            while i < len(s):
                if counter == 1 and s[i] == ")":
                    rvalue = calculate(substr)
                    break
                if s[i] == "(":
                    counter += 1
                if s[i] == ")":
                    counter -= 1
                substr += s[i]
                i += 1
        elif s[i].isnumeric():
            rstring = s[i]
            i += 1
            while i < len(s) and s[i].isnumeric():
                rstring += s[i]
                i += 1
            rvalue = int(rstring)
        if operator == "*":
            value *= rvalue
        if operator == "+":
            value += rvalue
        if i < len(s):
            operator = s[i]
        i += 1
    return value

def add_parenthesis(s):
    new_s = ""
    i = 0
    while i < len(s):
        if s[i] == "+":
            i += 1
            rvalue = ""
            if s[i] == "(":
                counter = 1
                i += 1
                substr = ""
                while i < len(s):
                    if counter == 1 and s[i] == ")":
                        rvalue = "(" + add_parenthesis(substr)
                        break
                    if s[i] == "(":
                        counter += 1
                    if s[i] == ")":
                        counter -= 1
                    substr += s[i]
                    i += 1
            elif s[i].isnumeric():
                rvalue = s[i]
                i += 1
                while i < len(s) and s[i].isnumeric():
                    rvalue += s[i]
                    i += 1

            j = len(new_s) - 1
            if new_s[j] == ")":
                counter = 1
                j -= 1
                substr = ""
                while j > 0:
                    if counter == 1 and new_s[j] == "(":
                        j -= 1
                        break
                    if new_s[j] == ")":
                        counter += 1
                    if new_s[j] == "(":
                        counter -= 1
                    j -= 1
            elif new_s[j].isnumeric():
                j -= 1
                while j > 0 and new_s[j].isnumeric():
                    j -= 1
            new_s = new_s[:j + 1] + "(" + new_s[j + 1:] + "+" + rvalue + ")"
            
        else:
            new_s += s[i]
            i += 1 
    return new_s

def part1(data):
    sumation = 0
    for line in data:
        line = line.replace(" ", "")
        sumation += calculate(line[:-1])
    return sumation

def part2(data):
    sumation = 0
    for line in data:
        line = line.replace(" ", "")
        sumation += calculate(add_parenthesis(line[:-1]))
    return sumation

if __name__ == '__main__':
    data = load_data("input")

    print("part1:", part1(data))
    print("part2:", part2(data))
