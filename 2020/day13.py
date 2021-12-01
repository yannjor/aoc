from math import ceil


def part1(startTime, buses):
    busIDs = [int(busID) for busID in buses if busID != "x"]
    closeTimes = [(busID, ceil(startTime / busID) * busID)
                  for busID in busIDs]
    busID, closestTime = min(closeTimes, key=lambda x: x[1])
    waitTime = closestTime - startTime
    return busID * waitTime


def main():
    with open("13.txt", "r") as f:
        lines = [line.strip() for line in f.readlines()]
        buses = lines[1].split(",")
        print("Part 1:", part1(int(lines[0]), buses))


if __name__ == "__main__":
    main()
