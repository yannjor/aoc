import re


def createBagDict(lines):
    bagDict = {}
    for line in lines:
        bag = line.split("bag")[0].strip()
        pattern = re.compile(r"(?<=\d\s)[a-z]+\s[a-z]+")
        amounts = list(map(lambda num: int(num), re.findall(r"\d", line)))
        contents = re.findall(pattern, line)
        inner = dict(zip(contents, amounts))
        bagDict[bag] = inner
    return bagDict


def part1(bagDict):
    def canHoldShinyGold(bags):
        if bags == {}:
            return False
        elif "shiny gold" in bags:
            return True
        else:
            canHold = False
            for key in bags.keys():
                canHold = canHold or canHoldShinyGold(bagDict[key])
            return canHold

    count = 0
    for val in bagDict.values():
        if canHoldShinyGold(val):
            count += 1
    return count


def part2(bagDict):
    sg = bagDict["shiny gold"]
    def bagsIn(bag):
        count = 0
        for key, val in bag.items():
            count += val + val * bagsIn(bagDict[key])
        return count
    return bagsIn(sg)


def main():
    with open("07.txt", "r") as f:
        content = f.readlines()
        bagDict = createBagDict(content)
        print("Part 1: ", part1(bagDict))
        print("Part 2: ", part2(bagDict))


if __name__ == "__main__":
    main()
