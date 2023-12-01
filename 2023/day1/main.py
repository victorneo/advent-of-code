import re


def solve_part1():
    f = open("input.txt")
    lines = f.read().split("\n")
    total = 0
    prog = re.compile(r"(\d)")
    for l in lines:
        groups = prog.findall(l)
        total += int(groups[0] + groups[-1])

    print(total)


def get_str_number(number):
    match number:
        case "one":
            return "1"
        case "two":
            return "2"
        case "three":
            return "3"
        case "four":
            return "4"
        case "five":
            return "5"
        case "six":
            return "6"
        case "seven":
            return "7"
        case "eight":
            return "8"
        case "nine":
            return "9"
        case _:
            return number


def solve_part2():
    f = open("input.txt")
    lines = f.read().split("\n")

    regex = r"(?=(\d|one|two|three|four|five|six|seven|eight|nine))"
    total = 0
    prog = re.compile(regex)
    for l in lines:
        groups = prog.findall(l)
        total += int(get_str_number(groups[0]) + get_str_number(groups[-1]))

    print(total)


if __name__ == "__main__":
    solve_part1()
    solve_part2()
