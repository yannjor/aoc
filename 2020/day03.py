def countTreeEncounters(lines, right, down):
    count = 0
    currentX = right
    currentY = down
    while currentY < len(lines):
        line = lines[currentY].strip()
        if line[currentX] == '#':
            count += 1
        currentX = (currentX + right) % len(line)
        currentY += down
    return count

def main():
    with open("03.txt") as f:
        content = f.readlines()
    # Part 1
    print(countTreeEncounters(content, 3, 1))
    # Part 2
    print(countTreeEncounters(content, 1, 1) *
          countTreeEncounters(content, 3, 1) *
          countTreeEncounters(content, 5, 1) *
          countTreeEncounters(content, 7, 1) *
          countTreeEncounters(content, 1, 2))


if __name__ == "__main__":
    main()
