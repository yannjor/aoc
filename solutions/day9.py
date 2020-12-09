def part1(nums, preambleLength):
    preamble = nums[:preambleLength]
    idx = 0
    for num in nums[preambleLength:]:
        found = False
        for i in range(preambleLength):
            for j in range(i+1, preambleLength):
                if preamble[i] + preamble[j] == num:
                    found = True
                    break
        if not found:
            return num
        idx += 1
        preamble = nums[idx:preambleLength+idx]
    return None


def part2(nums, target):
    for i in range(len(nums)):
        startSum = nums[i]
        for j in range(i+1, len(nums)):
            startSum += nums[j]
            if startSum == target:
                return min(nums[i:j+1]) + max(nums[i:j+1])
            if startSum > target:
                break
    return None


def main():
    with open("09.txt", "r") as f:
        nums = [int(num.strip()) for num in f.readlines()]
        part1Sol = part1(nums, 25)
        print("Part 1:", part1Sol)
        print("Part 2:", part2(nums, part1Sol))


if __name__ == "__main__":
    main()
