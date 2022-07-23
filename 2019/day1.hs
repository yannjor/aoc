module Day1 where

calculateFuel :: Int -> Int
calculateFuel m = (m `div` 3) - 2

partOne :: [Int] -> Int
partOne xs = sum (map calculateFuel xs)

fuelCounts :: Int -> [Int]
fuelCounts x | f < 0     = []
             | otherwise = f : fuelCounts f
  where f = calculateFuel x

partTwo :: [Int] -> Int
partTwo xs = sum (map (sum . fuelCounts) xs)

main :: IO ()
main = do
  content <- readFile "./inputs/day1.txt"
  let masses = map read (lines content) :: [Int]
  putStrLn ("Part 1: " ++ show (partOne masses))
  putStrLn ("Part 2: " ++ show (partTwo masses))
