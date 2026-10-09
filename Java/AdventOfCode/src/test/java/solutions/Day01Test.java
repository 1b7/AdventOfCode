package solutions;

import common.Solution;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;
import support.SampleDataProvider;

import static org.assertj.core.api.Assertions.assertThat;

public class Day01Test {

    private static Solution underTest;

    @BeforeAll
    static void setup() {
        underTest = new Day01();
    }

    @Test
    void partOne_passesOnSmallExample() {
        // Given
        String input = SampleDataProvider.getSampleInputAsString(1);

        // When
        long answer = underTest.partOne(input);

        // Then
        assertThat(answer).isEqualTo(3);
    }

    @Test
    void partTwo_passesOnSmallExample() {
        // Given
        String input = SampleDataProvider.getSampleInputAsString(1);

        // When
        long answer = underTest.partTwo(input);

        // Then
        assertThat(answer).isEqualTo(6);
    }
}
