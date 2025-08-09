import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.HashSet;
import java.util.List;
import java.util.Set;

public class Main {
    public static void main(String[] args) {
        WordSearch ws = new WordSearch("../../input/04");
        System.out.println("Part 1: " + ws.occurencesOf("XMAS"));
        System.out.println("Part 2: " + ws.crossesOfMas());
    }
}

class WordSearch {
    private char[][] search;

    public WordSearch(String filePath) {
        List<String> inputText;
        try {
            inputText = Files.readAllLines(Paths.get(filePath));
        } catch (IOException e) {
            System.err.println(e.getMessage());
            return;
        }

        this.search = inputText.stream()
                .map(line -> line.trim().toCharArray())
                .toArray(char[][]::new);
    }

    /**
     * Returns the number of instances of the given word within the word search.
     */
    public int occurencesOf(String word) {
        int occurrences = 0;

        for (int i = 0; i < search.length; i++) {
            for (int j = 0; j < search[i].length; j++) {
                if (checkRight(word, i, j)) occurrences++;
                if (checkDown(word, i, j)) occurrences++;
                if (checkLeft(word, i, j)) occurrences++;
                if (checkUp(word, i, j)) occurrences++;
                if (checkUpLeft(word, i, j)) occurrences++;
                if (checkUpRight(word, i, j)) occurrences++;
                if (checkDownLeft(word, i, j)) occurrences++;
                if (checkDownRight(word, i, j)) occurrences++;
            }
        }

        return occurrences;
    }

    private boolean checkRight(String target, int row, int col) {
        if (col + target.length() > search[row].length) return false;

        for (int i = 0; i < target.length(); i++) {
            if (target.charAt(i) != search[row][col + i]) return false;
        }

        return true;
    }

    private boolean checkLeft(String target, int row, int col) {
        if (col - (target.length() - 1) < 0) return false;

        for (int i = 0; i < target.length(); i++) {
            if (target.charAt(i) != search[row][col - i]) return false;
        }

        return true;
    }

    private boolean checkDown(String target, int row, int col) {
        if (row + target.length() > search.length) return false;

        for (int i = 0; i < target.length(); i++) {
            if (target.charAt(i) != search[row + i][col]) return false;
        }

        return true;
    }

    private boolean checkUp(String target, int row, int col) {
        if (row - (target.length() - 1) < 0) return false;

        for (int i = 0; i < target.length(); i++) {
            if (target.charAt(i) != search[row - i][col]) return false;
        }

        return true;
    }

    private boolean checkUpLeft(String target, int row, int col) {
        if (row - (target.length() - 1) < 0) return false;
        if (col - (target.length() - 1) < 0) return false;

        for (int i = 0; i < target.length(); i++) {
            if (target.charAt(i) != search[row - i][col - i]) return false;
        }

        return true;
    }

    private boolean checkUpRight(String target, int row, int col) {
        if (row - (target.length() - 1) < 0) return false;
        if (col + target.length() > search[row].length) return false;

        for (int i = 0; i < target.length(); i++) {
            if (target.charAt(i) != search[row - i][col + i]) return false;
        }

        return true;
    }

    private boolean checkDownLeft(String target, int row, int col) {
        if (row + target.length() > search.length) return false;
        if (col - (target.length() - 1) < 0) return false;

        for (int i = 0; i < target.length(); i++) {
            if (target.charAt(i) != search[row + i][col - i]) return false;
        }

        return true;
    }

    private boolean checkDownRight(String target, int row, int col) {
        if (row + target.length() > search.length) return false;
        if (col + target.length() > search[row].length) return false;

        for (int i = 0; i < target.length(); i++) {
            if (target.charAt(i) != search[row + i][col + i]) return false;
        }

        return true;
    }

    /**
     * Returns the number of 'X-MAS' instances within the word search.
     */
    public int crossesOfMas() {
        int count = 0;

        for (int i = 0; i < search.length; i++) {
            for (int j = 0; j < search[i].length; j++) {
                if (isMasCross(i, j)) count++;
            }
        }

        return count;
    }

    /**
     * Test if the given row/col pair is the centre of an 'X-MAS' instance.
     */
    private boolean isMasCross(int row, int col) {
        char centre = 'A';
        Set<Character> others = new HashSet<>();
        others.add('M');
        others.add('S');

        if (row < 1 || col < 1) return false;
        if (row >= search.length - 1 || col >= search[row].length - 1) return false;
        if (search[row][col] != centre) return false;

        return  // Test leading diagonal for { M, S } pair:
                others.contains(search[row - 1][col - 1]) &&
                others.contains(search[row + 1][col + 1]) &&
                search[row + 1][col + 1] != search[row - 1][col - 1] &&
                // Test opposite diagonal for { M, S } pair:
                others.contains(search[row - 1][col + 1]) &&
                others.contains(search[row + 1][col - 1]) &&
                search[row - 1][col + 1] != search[row + 1][col - 1];
                // If both are satisfied, returns true, else false.
    }
}