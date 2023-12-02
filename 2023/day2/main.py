def solve_part1_and_part2():
    f = open("input.txt")
    lines = f.read().split("\n")

    total = 0
    power = 0

    max = {
        "red": 12,
        "green": 13,
        "blue": 14,
    }

    for game in lines:
        game, rounds = game.strip().split(":")
        rounds = rounds.strip().split(";")
        id = int(game.strip()[5:])

        round_max = {
            "red": 0,
            "green": 0,
            "blue": 0,
        }

        # Keep track of the max number of each cube type

        for r in rounds:
            cubes = r.split(",")
            for cube in cubes:
                num, cube_type = cube.strip().split(" ")
                num = int(num)
                cube_type = cube_type.strip()

                if round_max[cube_type] < num:
                    round_max[cube_type] = num

        power += round_max["blue"] * round_max["green"] * round_max["red"]

        if (
            round_max["blue"] > max["blue"]
            or round_max["green"] > max["green"]
            or round_max["red"] > max["red"]
        ):
            continue
        else:
            total += id
    print(total)
    print(power)


if __name__ == "__main__":
    solve_part1_and_part2()
