import graphviz

if __name__ == "__main__":
  g = graphviz.Graph()
  with open("input") as fp:
    for line in fp.readlines():
      l, r = line.strip().split(': ')
      r = r.split(' ')
      [g.edge(l, x) for x in r]
  g.format = "svg"
  g.render()