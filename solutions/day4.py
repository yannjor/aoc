import re

def countValidPassports(passports):
    validCount1 = 0
    validCount2 = 0
    requiredFields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
    optionalFields = ["cid"]
    for passport in passports:
        fieldDict = dict(field.split(":") for field in passport.split(" "))
        fields = fieldDict.keys()
        if (sorted(fields) == sorted(requiredFields) or
                sorted(fields) == sorted(requiredFields + optionalFields)):
            validCount1 += 1
            if validateFields(fieldDict):
                validCount2 += 1
    return validCount1, validCount2


def validateFields(fields):
    byr = int(fields["byr"]) >= 1920 and int(fields["byr"]) <= 2002
    iyr = int(fields["iyr"]) >= 2010 and int(fields["iyr"]) <= 2020
    eyr = int(fields["eyr"]) >= 2020 and int(fields["eyr"]) <= 2030
    hgt = False
    if "cm" in fields["hgt"]:
        height = int(fields["hgt"].split("cm")[0])
        hgt = height >= 150 and height <= 193
    elif "in" in fields["hgt"]:
        height = int(fields["hgt"].split("in")[0])
        hgt = height >= 59 and height <= 76

    hcl = re.match(r"^#[0-9a-f]{6}$", fields["hcl"])
    ecl = fields["ecl"] in ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
    pid = re.match(r"^\d{9}$", fields["pid"])
    return byr and iyr and eyr and hgt and hcl and ecl and pid


def main():
    with open("04.txt") as f:
        content = f.read()
        passports = [line.replace("\n", " ").strip()
                     for line in content.split("\n\n")]
    print(countValidPassports(passports))


if __name__ == "__main__":
    main()
