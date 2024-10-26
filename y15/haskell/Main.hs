module Main where

main :: IO ()
main = undefined

d1_1 :: Num a => [Char] -> a
d1_1 xs = sum $ map value xs
  where
    value '(' = 1
    value ')' = -1
    value _ = 0

d1_2 :: String -> Int
d1_2 = aux 0
  where
    aux _ [] = 0
    aux n (x:xs)
      | n == basement = 0
      | otherwise = 1 + (aux (n + value x) xs)
    basement = -1
    value '(' = 1
    value ')' = -1
    value _ = 0

