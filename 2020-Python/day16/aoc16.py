def load_data(input):
    with open(input, "r") as file:
        return file.readlines()

def parse_ranges_tickets(data):
    part_of_data = 0
    ranges = []
    my_ticket = []
    tickets = []
    for line in data:
        if line == "\n":
            part_of_data += 1
            continue

        if part_of_data == 0:
            line = line.split(":")[1]
            line = line.split(" ")
            range1 = line[1].split("-")
            range2 = line[3].split("-")
            ranges.append((int(range1[0]), int(range1[1])))
            ranges.append((int(range2[0]), int(range2[1])))

        if part_of_data == 1:
            if line == "your ticket:\n":
                continue
            my_ticket = [int(number) for number in line.split(",")]

        if part_of_data == 2:
            if line == "nearby tickets:\n":
                continue
            ticket = [int(number) for number in line.split(",")]
            tickets.append(ticket)
    return ranges, my_ticket, tickets

def part1(ranges, my_ticket, tickets):
    data = load_data("input")
    
    error_rate = 0

    for ticket in tickets:
        for number in ticket:
            b = False
            for r in ranges:
                if r[0] <= number <= r[1]:
                    b = True
            if not b:
                error_rate += number

    return error_rate

def find_options_for_numbers(ranges, tickets):
    options = [[] for _ in range(len(ranges)//2)]
    for i in range(0, len(ranges), 2):
        r1 = ranges[i]
        r2 = ranges[i + 1]
        for j in range(len(tickets[0])):
            b = True
            for ticket in tickets:
                if not ((r1[0] <= ticket[j] <= r1[1]) or (r2[0] <= ticket[j] <= r2[1])):
                    b = False
            if b:
                options[i//2].append(j)
    return options

def part2(ranges, my_ticket, tickets):
    for ticket in tickets:
        for number in ticket:
            b = False
            for r in ranges:
                if r[0] <= number <= r[1]:
                    b = True
            if not b:
                tickets.remove(ticket)

    options = find_options_for_numbers(ranges, tickets)
    search = list(range(len(options)))
    while search:
        for i in search:
            if len(options[i]) == 1:
                found = options[i][0]
                search.remove(i)
                for j in search:
                    if found in options[j]:
                        options[j].remove(found)

            if len(options[i]) == 0:
                search.remove(i)

    multiply = 1
    for i in range(6):
        multiply *= my_ticket[options[i][0]]

    return multiply

if __name__ == '__main__':
    data = load_data("input")
    ranges, my_ticket, tickets = parse_ranges_tickets(data)
    print(part1(ranges, my_ticket, tickets))
    print(part2(ranges, my_ticket, tickets))
