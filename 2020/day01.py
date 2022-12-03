def partOne(entries):
    for i in range(len(entries)):
        for j in range(i + 1, len(entries)):
            if entries[i] + entries[j] == 2020:
                return entries[i] * entries[j]


def partTwo(entries):
    for i in range(len(entries)):
        for j in range(i + 1, len(entries)):
            for k in range(i + 2, len(entries)):
                if entries[i] + entries[j] + entries[k] == 2020:
                    return entries[i] * entries[j] * entries[k]


def main():
    with open("01.txt") as f:
        content = list(map(lambda numStr: int(numStr), f.readlines()))
    print(partOne(content))
    print(partTwo(content))


if __name__ == "__main__":
    main()
