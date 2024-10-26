module Main where

main :: IO ()
main = undefined

basement :: Int
basement = -1

value :: Char -> Int
value '(' = 1
value ')' = -1
value _ = 0

d1_1 :: [Char] -> Int
d1_1 xs = sum $ map value xs

d1_2 :: String -> Int
d1_2 = aux 0
  where
    aux _ [] = 0
    aux n (x:xs)
      | n == basement = 0
      | otherwise = 1 + (aux (n + value x) xs)

