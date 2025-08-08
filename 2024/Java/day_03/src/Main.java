import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class Main {

    /**
     * Given an input string, process valid multiplication instructions and add
     * up their results.
     */
    private static Integer processMultiplication(String input) {
        Pattern pattern = Pattern.compile("mul\\(\\d{1,3},\\d{1,3}\\)");
        Matcher matcher = pattern.matcher(input);

        return matcher.results()
                .map(result -> {
                    String match = result.group();
                    String[] nums = match.substring(4, match.length() - 1).split(",");
                    int x = Integer.parseInt(nums[0]);
                    int y = Integer.parseInt(nums[1]);
                    return x * y;
                })
                .reduce(0, Integer::sum);
    }

    /**
     * Remove disabled multiplications (those preceded by `don't()`) from a given
     * input string.
     */
    private static String removeDisabledMuls(String input) {
        // Iterate over the string, identifying regions of switched-off
        // multiplications by tracking appearances of `do()` and `don't()` substrings.
        // Once a region has been identified, it is 'sliced' out of the string.
        int i = 0;

        // If regionStart == -1, we are *not* in a disabled region,
        // if it's >= 0, we are currently in one.
        int regionStart = -1;

        while (i < input.length()) {
            if (input.startsWith("don't()", i) && regionStart == -1) {
                regionStart = i;
            }

            if (input.startsWith("do()", i) && regionStart != -1) {
                input = input.substring(0, regionStart) + input.substring(i);
                i = regionStart;
                regionStart = -1;
            }

            i++;
        }

        if (regionStart != -1) {
            input = input.substring(0, regionStart);
        }

        return input;
    }

    public static void main(String[] args) {
        Path inputPath = Paths.get("../../input/03");
        String input;

        try {
            input = Files.readString(inputPath);
        } catch (IOException e) {
            System.err.println(e.getMessage());
            return;
        }

        System.out.println("Part 1: " + processMultiplication(input));
        System.out.println("Part 2: " + processMultiplication(removeDisabledMuls(input)));
    }
}