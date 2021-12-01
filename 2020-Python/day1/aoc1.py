def load_numbers(file_name):
    numbers = []
    with open("input", "r") as file:

        for line in file:
            numbers.append(int(line[:-1]))
    return numbers

def part1(numbers):
    for number in numbers:
        for number2 in numbers:
                if number + number2 == 2020:
                    return number*number2
                    
def part2(numbers):
    for number in numbers:
        for number2 in numbers:
            for number3 in numbers:
                if number + number2 + number3 == 2020:
                    return number*number2*number3
                    
if __name__ == '__main__':
    numbers = load_numbers("input")

    print("part1:", part1(numbers))
    print("part2:", part2(numbers))
