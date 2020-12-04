def getValidCounts(lines):
    validCount1 = 0
    validCount2 = 0
    for line in lines:
        split = line.split(" ")
        nums = split[0].split("-")
        atLeast = int(nums[0])
        atMost = int(nums[1])
        letter = split[1][0]
        password = split[2]
        if (password.count(letter) >= atLeast and
                password.count(letter) <= atMost):
            validCount1 += 1
        if ((password[atLeast-1] == letter and password[atMost-1] != letter) or
                (password[atLeast-1] != letter and password[atMost-1] == letter)):
            validCount2 += 1
    return validCount1, validCount2


def main():
    with open("02.txt") as f:
        content = f.readlines()
    print(getValidCounts(content))


if __name__ == "__main__":
    main()
