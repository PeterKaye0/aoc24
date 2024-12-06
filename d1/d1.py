with open('day1_input','r+') as input_file:
    data = input_file.read().split('\n')

pairs = [data[i].split() for i in range(len(data))]
l1 =[int(pairs[x][0]) for x in range(len(pairs))]
l2 = [int(pairs[x][1]) for x in range(len(pairs))]

l1.sort()
l2.sort()

p1_ans = sum(abs(x-y) for x,y in zip(l1,l2))
p2_ans = sum(l2.count(i)*i for i in l1)

print("part 1 answer:",p1_ans)
print("part 2 answer:",p2_ans, )