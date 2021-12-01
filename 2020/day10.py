def part1(joltages):
    oneJoltDiffs = 0
    threeJoltDiffs = 1
    rating = 0
    for joltage in joltages:
        if joltage - 1 == rating:
            oneJoltDiffs += 1
        elif joltage - 3 == rating:
            threeJoltDiffs += 1
        rating = joltage
    return oneJoltDiffs * threeJoltDiffs


def part2(joltages):
    pathsToStep = {0: 1}
    for i in range(len(joltages)):
        for j in range(i+1, len(joltages)):
            if joltages[i] + 3 < joltages[j]:
                break
            if j in pathsToStep:
                pathsToStep[j] += pathsToStep[i]
            else:
                pathsToStep[j] = pathsToStep[i]
    return pathsToStep[len(joltages)-1]


def main():
    with open("10.txt", "r") as f:
        outletJoltage = 0
        joltages = [outletJoltage] + \
            sorted([int(line.strip()) for line in f.readlines()])
        print("Part 1:", part1(joltages))
        print("Part 2:", part2(joltages))


if __name__ == "__main__":
    main()
