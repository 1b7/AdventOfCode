package support;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;

public class SampleDataProvider {

    private static final String FILE_PATH_TEMPLATE = System.getProperty("user.home")
        + "/Dev/AdventOfCode/Java/AdventOfCode/src/test/resources/sample_inputs/%02d";

    private SampleDataProvider() {}
    public static String getSampleInputAsString(int day) {
        Path path = Path.of(FILE_PATH_TEMPLATE.formatted(day));
        try {
            return Files.readString(path);
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
    }
}
