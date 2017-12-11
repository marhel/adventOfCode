import Criterion.Main
import Test.Hspec
import Data.Char
import Data.List.Split

data Coord = Axial Int Int | Cubic Int Int Int

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

instance Show (Coord) where
   show (Axial q r) = "Axial " ++ show q ++ " " ++ show r
   show (Cubic x z y) = "Cubic " ++ show x ++ " " ++ show z ++ " " ++ show y

instance Eq Coord where
  Axial q1 r1 == Axial q2 r2 = q1 == q2 && r1 == r2
  Cubic x1 z1 y1 == Cubic x2 z2 y2 = x1 == x2 && z1 == z2 && y1 == y2

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
