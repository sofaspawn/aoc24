with open("input.txt") as f:
    l = [line.strip().split() for line in f if line.strip()]
    l = [x for x in l if len(x) == 2]
    l1 = sorted(int(x[0]) for x in l)
    l2 = sorted(int(x[1]) for x in l)
    ans = sum(abs(l1[i] - l2[i]) for i in range(len(l1)))
    print(ans)
