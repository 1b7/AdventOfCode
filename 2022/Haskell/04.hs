type RangePair = ((Int, Int), (Int, Int))

sepRanges :: [Char] -> [[Char]]
sepRanges [] = []
sepRanges s = part : sepRanges rest
    where part = takeWhile (`notElem` "-,") s
          rest = dropWhile (`elem` "-,") . dropWhile (`notElem` "-,") $ s

asRange :: String -> RangePair
asRange s = let [a, b, c, d] = map read $ sepRanges s in ((a, b), (c, d))

isSuperset :: RangePair -> Bool
isSuperset ((a, b), (x, y)) = (a <= x) && (b >= y) || (x <= a) && (y >= b)

isDisjoint :: RangePair -> Bool
isDisjoint ((a, b), (x, y)) = (a < x) && (b < x) || (a > y) && (b > y)

main = do
    input <- readFile "../input/04"
    let rs = map asRange $ lines input

    putStrLn . ("Part 1: " ++) . show . length . filter isSuperset $ rs
    putStrLn . ("Part 2: " ++) . show . length . filter (not . isDisjoint) $ rs