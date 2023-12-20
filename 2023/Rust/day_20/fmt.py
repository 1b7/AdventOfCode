with open("../../input/20") as fp:
  lines = fp.readlines()

chunks = [line.strip().split( " -> ") for line in lines]
for i in range(len(chunks)):
  chunk = chunks[i]
  chunk[1] = ','.join(f'"{s}"' for s in chunk[1].split(", "))

pref_names = [chunk[0] for chunk in chunks]
norm_names = [name.replace("%", "").replace("&", "") for name in pref_names]

name_pairs = list(zip(norm_names, pref_names))


new_lines = []
for chunk in chunks:
  for l, r in name_pairs:
    chunk[1] = chunk[1].replace(l, r)
  new_lines.append(f'"{chunk[0].replace("%", "F_").replace("&", "A_")}" -> {chunk[1].replace("%", "F_").replace("&", "A_")}')

print('\n'.join(new_lines))