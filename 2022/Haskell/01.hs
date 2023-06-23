import Data.List (sort)

splitIf _ [] = []
splitIf f xs = left : rest
    where left  = takeWhile f xs
          rest  = splitIf f . dropWhile (not . f) . dropWhile f $ xs

parse = map (sum . map read) . splitIf (not . null) . lines

main = do
    elves <- reverse . sort . parse <$> readFile "../input/01"
    print $ head elves
    print $ sum . take 3 $ elves