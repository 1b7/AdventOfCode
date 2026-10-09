package solutions;

import common.Solution;

import java.util.HashMap;
import java.util.List;

public class Day01 implements Solution {

  private final HashMap<String, Result> resultCache = new HashMap<>();

  @Override
  public long partOne(String input) {
    return getOrCompute(input).zeroesSeen();
  }

  @Override
  public long partTwo(String input) {
    return getOrCompute(input).zeroesPassed();
  }

  private Result getOrCompute(String input) {
    if (!resultCache.containsKey(input)) {
      compute(input);
    }

    return resultCache.get(input);
  }

  private void compute(String input) {
    List<Long> values = processInput(input);

    long zeroesSeen = 0;
    long zeroesPassed = 0;
    long dial = 50;

    for (Long n : values) {
      long delta = n % 100;

      if (dial != 0 && (dial + delta >= 100 || dial + delta <= 0)) {
        zeroesPassed += 1;
      }
      zeroesPassed += Math.abs(n) / 100;

      dial = (100 + delta + dial) % 100;

      if (dial == 0) {
        zeroesSeen += 1;
      }
    }

    this.resultCache.put(input, new Result(zeroesSeen, zeroesPassed));
  }

  List<Long> processInput(String input) {
    return input.lines()
        .map(this::lineToValue)
        .toList();
  }

  long lineToValue(String line) {
    long n = Long.parseLong(
        line.substring(1)
    );

    return (line.charAt(0) == 'L') ? n : -n;
  }

  private record Result(
      long zeroesSeen,
      long zeroesPassed
  ) {}
}
