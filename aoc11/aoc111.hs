import Criterion.Main
import Test.Hspec
import Data.Char
import Data.List.Split

data Coord = Axial Int Int | Cubic Int Int Int deriving (Eq, Show)

twist :: Coord -> Coord
twist (Axial q r) = Cubic q r ((-q) - r)
twist (Cubic x z _) = Axial x z

step :: Coord -> String -> Coord
step (Axial q r) "n" = Axial q (r - 1)
step (Axial q r) "s" = Axial q (r + 1)
step (Axial q r) "ne" = Axial (q + 1) (r - 1)
step (Axial q r) "nw" = Axial (q - 1) r
step (Axial q r) "se" = Axial (q + 1) r
step (Axial q r) "sw" = Axial (q - 1) (r + 1)
step (Axial q r) _ = Axial q r

distance :: Coord -> Int
distance a@(Axial _ _) = distance (twist a)
distance (Cubic x z y) = foldl max 0 $ map abs [x, z, y]

walkDistance :: String -> Int
walkDistance steps = distance $ foldl step (Axial 0 0) (splitOn "," steps)

maxWalkDistance :: String -> Int
maxWalkDistance steps = foldl max 0 $ map distance $ scanl step (Axial 0 0) (splitOn "," steps)

main = do
    test
    text <- readFile "input.txt"
    defaultMain
        [ bench "aoc11.2" $ whnf maxWalkDistance text
        , bench "aoc11.1" $ whnf walkDistance text
        ]

test :: IO ()
test = hspec $ do
  describe "Day 11: Hex Ed" $ do
    it "can step north" $ do
        step (Axial 0 0) "n" `shouldBe` (Axial 0 (-1))
    it "can step northeast" $ do
        step (Axial 0 0) "ne" `shouldBe` (Axial 1 (-1))
    it "can step northwest" $ do
        step (Axial 0 0) "nw" `shouldBe` (Axial (-1) 0)
    it "can step south" $ do
        step (Axial 0 0) "s" `shouldBe` (Axial 0 1)
    it "can step southeast" $ do
        step (Axial 0 0) "se" `shouldBe` (Axial 1 0)
    it "can step southwest" $ do
        step (Axial 0 0) "sw" `shouldBe` (Axial (-1) 1)
    it "can split a string on comma" $ do
        splitOn "," "se,sw,se,sw,sw" `shouldBe` (["se", "sw", "se", "sw", "sw"])
    it "can twist from Cubic coords" $ do
        twist (Cubic 2 3 (-5)) `shouldBe` (Axial 2 3)
    it "can twist from Axial coords" $ do
        twist (Axial 2 3) `shouldBe` (Cubic 2 3 (-5))
    it "can calculate distance from starting point" $ do
        walkDistance "se,sw,se,sw,sw" `shouldBe` (3 :: Int)
        walkDistance "ne,ne,s,s" `shouldBe` (2 :: Int)
        walkDistance "ne,ne,sw,sw" `shouldBe` (0 :: Int)
        walkDistance "ne,ne,ne" `shouldBe` (3 :: Int)
    it "can calculate max distance from starting point" $ do
        maxWalkDistance "se,sw,se,sw,sw,ne,ne,nw,ne" `shouldBe` (3 :: Int)
        maxWalkDistance "ne,ne,sw,sw" `shouldBe` (2 :: Int)

readAndProcess f fileName = do
  text <- readFile fileName
  return $ f text
