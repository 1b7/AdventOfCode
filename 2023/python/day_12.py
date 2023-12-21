from functools import cache

# Principle idea is to recursively identify and test placements for blocks of damaged springs. 
# Caching is used to make this significantly more efficient!
def perms(orig_springs, orig_seqs):

    # nonlocal is used to provide access to the springs/seqs variables without
    # having to pass them into the function `f` - which would prevent caching.
    # `perms` is effectively just acting as a convenient wrapper over `f`.
    @cache
    def f(spring_ptr, seq_ptr):
        nonlocal orig_springs
        nonlocal orig_seqs
        springs = orig_springs[spring_ptr:]
        seqs = orig_seqs[seq_ptr:]

        # Base Cases, exhausted the input:
        if len(seqs) == 0:
            return 0 if any(c == '#' for c in springs) else 1
        if len(springs) == 0: return 0

        # Get the location of the first known-damaged spring in this chunk of input;
        # We can't start a window beyond this point, else we would be skipping over
        # damaged springs without counting them in a sequenceL
        next_damaged = next((i + 1 for i, c in enumerate(springs) if c == '#'), len(springs))

        # Use a sliding window over the input (starting from the last split point),
        # If a valid location is found, recursively find placements for the
        # next block of damaged springs from that point.
        seq_size = seqs[0]
        count = 0
        for i in range(next_damaged):
            end = (i + seq_size)

            # Skip windows which would run past the end of the input:
            if end > len(springs): break
            # Make sure we aren't starting immediately after, or ending immediately
            # before a damaged spring (as this would become one contiguous block):
            if i > 0 and springs[i - 1] == '#': continue
            if end < len(springs) and springs[end] == '#': continue 

            # Finally, if the window only includes unknown/damaged springs, it must
            # be a valid placement for a damaged block of this size; recurse from here:
            if all(c == '#' or c == '?' for c in springs[i:end]):
                count += f(spring_ptr + end + 1, seq_ptr + 1)
        
        return count
    return f(0, 0)

# Get a row of input text into a form we can use:
def fmt(s):
    springs, seq = s.split()
    seq = [int(n) for n in seq.split(',')]
    return (springs, seq)

# For part 2, unfold one row of the input:
def unfold(parsed_txt):
    return [('?'.join([springs] * 5), seqs * 5)  for springs, seqs in parsed_txt]

if __name__ == "__main__":
    with open("../../input/12") as fp: txt = fp.readlines()

    p1_input = [fmt(t) for t in txt]
    p1_ans = sum(perms(row[0], row[1]) for row in p1_input)
    print(f"Part 1: {p1_ans}")

    p2_input = unfold(p1_input)
    p2_ans = sum(perms(row[0], row[1]) for row in p2_input)
    print(f"Part 2: {p2_ans}")