from functools import reduce


def getNeighbors(grid, x, y):
    directions = [(0, 1), (1, 0), (-1, 0), (0, -1),
                  (1, 1), (1, -1), (-1, 1), (-1, -1)]
    neighbors = []
    visibleNeighbors = []
    for direction in directions:
        dx, dy = direction
        if (x + dx < 0 or x + dx >= len(grid[0]) or
                y + dy < 0 or y + dy >= len(grid)):
            continue
        neighbors.append(grid[y+dy][x+dx])
        visibleNeighbor = findSeatInDirection(grid, x, y, dx, dy)
        if visibleNeighbor is not None:
            visibleNeighbors.append(visibleNeighbor)
    return neighbors, visibleNeighbors


def findSeatInDirection(grid, x0, y0, xDir, yDir):
    x1 = x0+xDir
    y1 = y0+yDir
    while (0 <= x1 < len(grid[0]) and 0 <= y1 < len(grid)):
        if grid[y1][x1] != ".":
            return grid[y1][x1]
        x1 += xDir
        y1 += yDir
    return None


def step(grid, minOccupied, visibleNeighbors):
    newGrid = []
    for i in range(len(grid)):
        newGrid.append("")
        for j in range(len(grid[i])):
            adjacent, visible = getNeighbors(grid, j, i)
            neighbors = visible if visibleNeighbors else adjacent
            if grid[i][j] == "L" and "#" not in neighbors:
                newGrid[i] += "#"
            elif grid[i][j] == "#" and neighbors.count("#") >= minOccupied:
                newGrid[i] += "L"
            else:
                newGrid[i] += grid[i][j]
    return newGrid


def countOccupied(grid, minOccupied, visibleNeighbors):
    prevGrid = grid
    newGrid = step(grid, minOccupied, visibleNeighbors)
    while (newGrid != prevGrid):
        prevGrid = step(prevGrid, minOccupied, visibleNeighbors)
        newGrid = step(newGrid, minOccupied, visibleNeighbors)
    return reduce(lambda count, line: count + line.count("#"), newGrid, 0)


def main():
    with open("11.txt", "r") as f:
        lines = [line.strip() for line in f.readlines()]
        print("Part 1:", countOccupied(lines, 4, False))
        print("Part 2:", countOccupied(lines, 5, True))


if __name__ == "__main__":
    main()
