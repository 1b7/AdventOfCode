import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.*;

public class Main {
    public static void main(String[] args) {
        var queue = new PrintQueue("../../input/05");
        System.out.println("Part 1: " + queue.sumValidUpdates());
        System.out.println("Part 2: " + queue.correctInvalidUpdates());
    }
}

class PrintQueue {
    HashMap<Integer, HashSet<Integer>> rules;
    List<List<Integer>> updates;

    public PrintQueue(String dataPath) {
        try {
            String allData = Files.readString(Paths.get(dataPath));
            String[] segments =  allData.split("\n\n");
            this.rules = parseRules(segments[0]);
            this.updates = parseUpdates(segments[1]);
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
    }

    /**
     * Sums up the middle-elements of valid updates.
     */
    public int sumValidUpdates() {
        int sum = 0;

        for (var update : updates) {
            if (validateUpdate(update)) sum += update.get(update.size() / 2);
        }

        return sum;
    }

    /**
     * Corrects the ordering of invalid updates and sums up their middle-elements.
     */
    public int correctInvalidUpdates() {
        int sum = 0;

        for (List<Integer> update : updates) {
            if (!validateUpdate(update)) {
                List<Integer> fixed = fixUpdate(update);
                sum += fixed.get(fixed.size() / 2);
            }
        }

        return sum;
    }

    /**
     * Attempts to fix an invalid update by reconstructing it bottom-up, by finding
     * the first page that doesn't have to come before others, then the next one,
     * and so on.
     * @param update The update to be fixed.
     * @return A new list containing the correct ordering of elements.
     */
    private List<Integer> fixUpdate(List<Integer> update) {
        List<Integer> placed = new ArrayList<>();
        HashSet<Integer> remain = new HashSet<>(update);

        while (!remain.isEmpty()) {
            Optional<Integer> n = remain.stream()
                    .filter(m -> {
                        var intersection = new HashSet<>(rules.getOrDefault(m, new HashSet<>()));
                        intersection.retainAll(remain);
                        return intersection.isEmpty();
                    }).findFirst();

            placed.add(0, n.orElseThrow());
            remain.remove(n.orElseThrow());
        }

        return placed;
    }

    /**
     * Tests whether an update is valid or not, according to the given precedence
     * rules.
     */
    private boolean validateUpdate(List<Integer> update) {
        for (int i = 0; i < update.size(); i++) {
            int n = update.get(i);
            HashSet<Integer> mustPrecede = rules.get(n);
            if (mustPrecede == null) continue;

            for (int j = 0; j < i; j++) {
                if (mustPrecede.contains(update.get(j))) { return false; }
            }
        }

        return true;
    }

    /**
     * Parses text containing a series of precedence rules into a HashMap of
     * key-value pairs, in which an integer key must come before its associated
     * set of integer values in an update.
     * @param textRules The text to be parsed.
     * @return A HashMap of the precedence rules.
     */
    private static HashMap<Integer, HashSet<Integer>> parseRules(String textRules) {
        HashMap<Integer, HashSet<Integer>> rules = new HashMap<>();

        textRules.lines().forEach(line -> {
            if (line.isEmpty() || line.contains(",")) return;
            String[] chunks = line.trim().split("\\|");
            int left = Integer.parseInt(chunks[0]);
            int right = Integer.parseInt(chunks[1]);

            if (!rules.containsKey(left)) {
                rules.put(left, new HashSet<>());
            }

            rules.get(left).add(right);
        });

        return rules;
    }

    /**
     * Parses text containing multiple lines of "updates" into a list of updates
     * consisting of integers.
     * @param textUpdates The text to be parsed.
     * @return A list of the parsed updates.
     */
    private static List<List<Integer>> parseUpdates(String textUpdates) {
        List<List<Integer>> updates = new ArrayList<>();

        textUpdates.lines().forEach(line -> {
            updates.add(
                Arrays.stream(line.trim().split(","))
                    .map(Integer::parseInt)
                    .toList()
            );
        });

        return updates;
    }
}