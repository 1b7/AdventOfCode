import Data.Char

toPriority :: Char -> Int
toPriority c
    | c >= 'a'  = ord c - ord 'a' + 1
    | otherwise = ord c - ord 'A' + 27

hasRepeats :: (Foldable t, Eq a) => t a -> [a] -> [a]
hasRepeats a = filter (`elem` a)

halve :: [a] -> ([a], [a])
halve s = splitAt (length s `div` 2) s

duplicatedValue :: [Char] -> Int
duplicatedValue s = toPriority . head . uncurry hasRepeats $ halve s

getBadgeValue :: [[Char]] -> Int
getBadgeValue [s, t, u] = toPriority . head . filter (\ c -> c `elem` t && c `elem` u) $ s
getBadgeValue _ = error "Number of strings passed to function /= 3"

repeatBadges :: [String] -> Int
repeatBadges [] = 0
repeatBadges ss = getBadgeValue (take 3 ss) + repeatBadges (drop 3 ss)

main :: IO ()
main = do
    input <- readFile "../input/03"
    putStrLn . ("Part 1: " ++) . show . sum . map duplicatedValue . lines $ input
    putStrLn . ("Part 2: " ++) . show . repeatBadges . lines $ input