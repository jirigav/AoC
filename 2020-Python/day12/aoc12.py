def load_data(filename):
    data = []
    with open(filename, "r") as file:
        for line in file:
            data.append((line[0], int(line[1:])))

    return data

def change_coordinates(c, direction, distance):
    if direction == "N":
        c[0] += distance

    if direction == "S":
        c[0] -= distance

    if direction == "E":
        c[1] += distance

    if direction == "W":
        c[1] -= distance
    return c[0], c[1]

def part1(data):
    facing = "E"
    l90 = {"N" : "W", "W" : "S", "S" : "E", "E" : "N"}
    lr180 = {"N" : "S", "E" : "W", "W" : "E", "S" : "N"}
    r90 = {"N" : "E", "E" : "S", "S" : "W", "W" : "N"}
    n = 0
    e = 0
    for (direction, dist) in data:
        if direction == "F":
            direction = facing

        n,e = change_coordinates([n, e], direction, dist)

        if direction == "L" or direction == "R":
            degree = dist%360
            if degree == 180:
                facing = lr180[facing]
            elif (direction == "L" and degree  == 90) or (direction == "R" and degree == 270):
                facing = l90[facing]
            elif (direction == "R" and degree  == 90) or (direction == "L" and degree == 270):
                facing = r90[facing]

    return abs(n) + abs(e)

def part2(data):
    wpn = 1
    wpe = 10
    posn = 0
    pose = 0
    for (direction, dist) in data:
        if direction == "F":
            posn += dist*wpn
            pose += dist*wpe

        wpn, wpe = change_coordinates([wpn, wpe], direction, dist)

        if direction == "L" or direction == "R":
            degree = dist%360
            if degree == 180:
                wpn = -wpn
                wpe = -wpe
            elif (direction == "L" and degree  == 90) or (direction == "R" and degree == 270):
                wpe, wpn = -wpn, wpe
            elif (direction == "R" and degree  == 90) or (direction == "L" and degree == 270):
                wpe, wpn = wpn, -wpe

    return abs(posn) + abs(pose)

if __name__ == '__main__':
    data = load_data("input")
    print("part1:", part1(data))
    print("part2:", part2(data))



