import common.Solution;
import solutions.Day01;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;

public class Main {

  static void main(String[] args) {
    Solution day1 = new Day01();

    long result1 = day1.partOne(getInputAsString(1));
    System.out.println(result1);

    long result2 = day1.partTwo(getInputAsString(1));
    System.out.println(result2);
  }

  private static final String FILE_PATH_TEMPLATE = System.getProperty("user.home")
      + "/Dev/AdventOfCode/Java/AdventOfCode/src/test/resources/sample_inputs/%02d";

  public static String getInputAsString(int day) {
    Path path = Path.of(FILE_PATH_TEMPLATE.formatted(day));
    try {
      return Files.readString(path);
    } catch (IOException e) {
      throw new RuntimeException(e);
    }
  }
}
