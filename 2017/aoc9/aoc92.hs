module AoC9 where

import Test.Hspec
import Data.Char

data State = Normal Int Int | Garbage Int Int

accepts :: State -> String -> Int
accepts (Normal score x) ('{':xs) = accepts (Normal score (x + 1)) xs
accepts (Normal score 0) ('}':xs) = score
accepts (Normal score x) ('}':xs) = accepts (Normal score (x - 1)) xs
accepts state@(Normal _ _) (',':xs) = accepts state xs
accepts (Normal score x) ('<':xs) = accepts (Garbage score x) xs
accepts state@(Garbage _ _) ('!':_:xs) = accepts state xs
accepts (Garbage score x) ('>':xs) = accepts (Normal score x) xs
accepts state@(Garbage score x) (_:xs) = accepts (Garbage (score + 1) x) xs
accepts (Normal score x) _ = score

process :: String -> Int
process = accepts (Normal 0 1)

main :: IO ()
main = hspec $ do
  describe "Day 9: Stream Processing" $ do
    it "can parse a single group" $ do
        process "{}" `shouldBe` (0 :: Int) -- [1]
    it "can parse nested groups" $ do
        process "{{{}}}" `shouldBe` (0 :: Int) -- [1, 2, 3]
    it "can parse comma separated groups" $ do
        process "{{},{}}" `shouldBe` (0 :: Int) -- [1, 2, 2]
    it "can parse a combination of nested and separated groups" $ do
        process "{{{},{},{{}}}}" `shouldBe` (0 :: Int) -- [1, 2, 3, 3, 3, 4]
    it "can parse group with garbage" $ do
        process "{<a>,<a>,<a>,<a>}" `shouldBe` (4 :: Int) -- [1]
    it "can parse nested group with garbage" $ do
        process "{{<ab>},{<ab>},{<ab>},{<ab>}}" `shouldBe` (8 :: Int) -- [1, 2, 2, 2, 2]
    it "can parse escaped escapes" $ do
        process "{{<!!>},{<!!>},{<!!>},{<!!>}}" `shouldBe` (0 :: Int) -- [1, 2, 2, 2, 2]
    it "can parse escapes" $ do
        process "{{<a!>},{<a!>},{<a!>},{<ab>}}" `shouldBe` (17 :: Int) -- 1, 2
    it "can parse simple garbage" $ do
        process "{<>}" `shouldBe` (0 :: Int) -- [1]
    it "can parse random garbage" $ do
        process "{<()/&%€>}" `shouldBe` (6 :: Int) -- [1]
    it "can parse restarting garbage" $ do
        process "{<<<<>}" `shouldBe` (3 :: Int) -- [1]
    it "can parse group-like garbage" $ do
        process "{<{,{{}}]}}>}" `shouldBe` (9 :: Int) -- [1]
    it "can parse escaped garbage" $ do
        process "{<{!>}>}" `shouldBe` (2 :: Int) -- [1]
    it "can parse escaped escaped garbage" $ do
        process "{<!!!>>}" `shouldBe` (0 :: Int) -- [1]
    it "can parse other escaped garbage" $ do
        process "{<{o\"i!a,<{i<a>}" `shouldBe` (10 :: Int) -- [1]

readAndProcess fileName = do
    text <- readFile fileName
    return $ process text
