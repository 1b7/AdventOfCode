def run(fp):
    prios = 0
    with open(fp) as fp:
        for line in fp.readlines():
            left, right = line[len(line) // 2:].strip(), line[:len(line) // 2].strip()
            for c in left:
                if c in right:
                    prios += 1 + ((ord(c) - ord('a')) if c > 'a' else (ord(c) - ord('A') + 26))
                    break
    return prios

def badges(fp):
    prios = 0
    with open(fp) as fp:
        lines = fp.readlines()
        for i in range(0,len(lines),3):
            ls = lines[i:i+3]
            for a in ls[0]:
                if a in ls[1] and a in ls[2]:
                    prios += 1 + ((ord(a) - ord('a')) if a > 'a' else (ord(a) - ord('A') + 26))
                    break
    return prios

if __name__ == "__main__":
    print(run("../../input/03"))
    print(badges("../../input/03"))