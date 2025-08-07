import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.Arrays;
import java.util.List;

enum SafetyStatus { Safe, Dampened, Unsafe }

public class Main {
    private static SafetyStatus isSafeReport(Integer[] nums, boolean dampen) {
        // Capture the typical ordering (ascending or descending) of the list:
        int descents = 2;
        for (int i = 0; i < nums.length - 1 && i < 4; i++) {
            descents += (nums[i] > nums[i + 1]) ? 1 : -1;
        }
        boolean isAscending = descents <= 0;

        // Test the provided list, making a note of where (if anywhere) it fails
        // the tests:
        int unsafeIdx = -1;
        for (int i = 0; i < nums.length - 1; i++) {
            boolean validOrder = (nums[i] < nums[i + 1] && isAscending) ||
                    (nums[i] > nums[i + 1] && !isAscending);
            if (!validOrder) { unsafeIdx = i; break; }

            boolean validDelta = Math.abs(nums[i] - nums[i + 1]) <= 3 &&
                    nums[i] - nums[i + 1] != 0;;
            if (!validDelta) { unsafeIdx = i; break; }
        }

        if (dampen && unsafeIdx != -1) {
            // If dampening is allowed and the report is unsafe, try to fix it
            // by clipping out the error -- clipping either the 'i' on which the
            // error occurred *or* the following value might resolve the problem.

            // Clip the first value:
            Integer[] altered = new Integer[nums.length - 1];
            System.arraycopy(nums, 0, altered, 0, unsafeIdx);
            System.arraycopy(nums, unsafeIdx + 1, altered, unsafeIdx, nums.length - unsafeIdx - 1);

            if (isSafeReport(altered, false) == SafetyStatus.Safe) {
                return SafetyStatus.Dampened;
            }

            // If that didn't work, try clipping the second:
            unsafeIdx += 1;
            System.arraycopy(nums, 0, altered, 0, unsafeIdx);
            System.arraycopy(nums, unsafeIdx + 1, altered, unsafeIdx, nums.length - unsafeIdx - 1);

            return isSafeReport(altered, false) == SafetyStatus.Safe
                    ? SafetyStatus.Dampened
                    : SafetyStatus.Unsafe;
        } else {
            return unsafeIdx == -1 ? SafetyStatus.Safe : SafetyStatus.Unsafe;
        }
    }

    public static void main(String[] args) throws IOException {
        Path inputPath = Paths.get("../../input/02");
        List<String> lines = Files.readAllLines(inputPath);

        int safeReports = 0;
        int dampenedReports = 0;

        for (String line : lines) {
            Integer[] nums = Arrays.stream(line.split("\\s+"))
                .map(Integer::parseInt)
                .toArray(Integer[]::new);

            switch (isSafeReport(nums, true)) {
                case Safe -> safeReports++;
                case Dampened -> dampenedReports++;
            }
        }

        System.out.println("Part 1: " + safeReports);
        System.out.println("Part 2: " + (safeReports + dampenedReports));
    }
}