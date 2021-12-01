import re

def string_in_range(string, rng):
    if not string.isnumeric():
        return False
    return int(string) in rng

def correct_fields_in_batch(dict):
    for filed in ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]:
        if filed not in dict:
            return False
    return True

def correct_ecl(col):
    return col in ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]

def correct_hcl(hcl):
    return hcl[0] == "#" and len(hcl) == 7 and (re.fullmatch(r"^[0-9a-f]+$", hcl[1:]) is not None)

def correct_hgt(hgt):
    if "in" in hgt:
        return hgt[:-2].isnumeric() and int(hgt[:-2]) in range(59, 77)

    return hgt[:-2].isnumeric() and int(hgt[:-2]) in range(150, 194)

def part2_check(dict):
    if not correct_fields_in_batch(dict):
        return False

    if not string_in_range(dict["byr"], range(1920, 2003)):
        return False

    if not string_in_range(dict["iyr"], range(2010, 2021)):
        return False

    if not string_in_range(dict["eyr"], range(2020, 2031)):
        return False

    if not correct_ecl(dict["ecl"]):
        return False

    if not correct_hcl(dict["hcl"]):
        return False

    if len(dict["pid"]) != 9 or not dict["pid"].isnumeric():
        return False

    if not correct_hgt(dict["hgt"]):
        return False

    return True


if __name__ == '__main__':

    count_part1 = 0
    count_part2 = 0
    with open("input", "r") as file:
        batch = {}
        for line in file:

            if line == "\n":
                if correct_fields_in_batch(batch):
                    count_part1 += 1

                if part2_check(batch):
                    count_part2 += 1          
                batch = {}

            else:
                for field in line[:-1].split(" "):
                    field = field.split(":")
                    batch[field[0]] = field[1]

    print("part1:", count_part1)
    print("part2:", count_part2)
