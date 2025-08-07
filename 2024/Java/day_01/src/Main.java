import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.*;

public class Main {
    public static void main(String[] args) {
        String filepath = "../../input/01";
        InputData input = new InputData(Paths.get(filepath));

        System.out.println("Part 1: " + partOne(input));
        System.out.println("Part 2: " + partTwo(input));
    }

    /**
     * Calculate the sum of pairwise differences between the two
     * ordered lists of the input data.
     */
    private static int partOne(InputData input) {
        List<Integer> left = input.getLeftList();
        List<Integer> right = input.getRightList();

        int distance = 0;
        for (int i = 0; i < left.size(); i++) {
            distance += Math.abs(left.get(i) - right.get(i));
        }

        return distance;
    }

    /**
     * Calculate the "similarity score" between the two
     * ordered lists of the input data.
     */
    private static int partTwo(InputData input) {
        List<Integer> left = input.getLeftList();
        List<Integer> right = input.getRightList();

        int similarity = 0;

        for (int n : left) {
            int count = 0;
            int idx = Collections.binarySearch(right, n);

            if (idx >= 0) {
                count += 1;

                // From the initial 'hit', count all others to the
                // left *and* right of it.
                int offset = 1;
                while (Objects.equals(right.get(idx + offset), n)) {
                    offset += 1;
                    count += 1;
                }

                offset = -1;
                while (right.get(idx + offset) == n) {
                    offset -= 1;
                    count += 1;
                }
            }

            similarity += n * count;
        }

        return similarity;
    }
}

class InputData {
    List<Integer> left;
    List<Integer> right;

    public InputData(java.nio.file.Path filepath) {
        left = new ArrayList<>();
        right = new ArrayList<>();

        // Parse numbers from input list:
        try {
            List<String> lines = Files.readAllLines(filepath);
            for (String line : lines) {
                String[] nums = line.split(" +");
                left.add(Integer.parseInt(nums[0]));
                right.add(Integer.parseInt(nums[1]));
            }
        } catch (IOException e) {
            System.err.println(e.getMessage());
            throw new RuntimeException(e);
        }

        Collections.sort(left);
        Collections.sort(right);
    }

    List<Integer> getLeftList() { return this.left; }
    List<Integer> getRightList() { return this.right; }
}