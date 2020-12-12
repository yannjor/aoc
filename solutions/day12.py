from math import cos, sin, radians


def degreesToDirection(degrees):
    directions = [(0, 1), (1, 1), (1, 0), (1, -1),
                  (0, -1), (-1, -1), (-1, 0), (-1, 1)]
    idx = round(degrees / (360. / len(directions)))
    return directions[idx % len(directions)]


def turn(degrees, posX, posY):
    rad = radians(degrees)
    return (int(round(cos(rad) * posX - sin(rad) * posY)),
            int(round(sin(rad) * posX + cos(rad) * posY)))


def part1(instructions):
    posX = 0
    posY = 0
    direction = 90
    for instruction in instructions:
        action, value = instruction[0], int(instruction[1:])
        if action == "N":
            posY += value
        elif action == "S":
            posY -= value
        elif action == "E":
            posX += value
        elif action == "W":
            posX -= value
        elif action == "L":
            direction -= value
        elif action == "R":
            direction += value
        elif action == "F":
            dx, dy = degreesToDirection(direction)
            posX += dx * value
            posY += dy * value
    return abs(posX) + abs(posY)


def part2(instructions):
    posX = 0
    posY = 0
    waypoint = (10, 1)
    for instruction in instructions:
        action, value = instruction[0], int(instruction[1:])
        if action == "N":
            waypoint = (waypoint[0], waypoint[1] + value)
        elif action == "S":
            waypoint = (waypoint[0], waypoint[1] - value)
        elif action == "E":
            waypoint = (waypoint[0] + value, waypoint[1])
        elif action == "W":
            waypoint = (waypoint[0] - value, waypoint[1])
        elif action == "L":
            waypoint = turn(value, waypoint[0], waypoint[1])
        elif action == "R":
            waypoint = turn(-value, waypoint[0], waypoint[1])
        elif action == "F":
            posX += waypoint[0] * value
            posY += waypoint[1] * value
    return abs(posX) + abs(posY)


def main():
    with open("12.txt", "r") as f:
        lines = [line.strip() for line in f.readlines()]
        print("Part 1:", part1(lines))
        print("Part 2:", part2(lines))


if __name__ == "__main__":
    main()
