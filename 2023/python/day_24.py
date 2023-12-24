import z3
import numpy as np

def parse_text(s):
  p, v = s.split(" @ ")
  p = [int(x) for x in p.split(", ")]
  v = [int(x) for x in v.split(", ")]
  return p + v

def solve_system(ax, ay, ax_c, ay_c, bx, by, bx_c, by_c):
  a = np.array([[ax_c, -bx_c], [ay_c, -by_c]])
  b = np.array([ax - bx, ay - by])

  try:
    solution = -np.linalg.solve(a, b)
    if solution[0] > 0 and solution[1] > 0:
      return (ax + ax_c * solution[0], by + by_c * solution[1])
  except:
    return None

def p1(input):
  bounds = 200000000000000, 400000000000000
  count = 0
  for i in range(len(input)):
    for j in range(i + 1, len(input)):
      a = input[i]
      b = input[j]
      r = solve_system(a[0], a[1], a[3], a[4], b[0], b[1], b[3], b[4])
      if r is not None and r[0] >= bounds[0] and r[0] <= bounds[1] and r[1] >= bounds[0] and r[1] <= bounds[1]:
        count += 1
      
  return count

def p2(input):
  rock = z3.RealVector('r', 6)
  time = z3.RealVector('t', 3)

  s = z3.Solver()
  s.add(*[rock[d] + rock[d+3] * t == hail[d] + hail[d+3] * t
          for t, hail in zip(time, input) for d in range(3)])
  s.check()

  return s.model().eval(sum(rock[:3]))

if __name__ == "__main__":
  with open("input") as fp:
    input = [parse_text(line.strip()) for line in fp.readlines()]
    print("Part 1:", p1(input))
    print("Part 2:", p2(input))
