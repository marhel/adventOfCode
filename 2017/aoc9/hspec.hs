module MathSpec where

import Test.Hspec
{-
import Math
-}

main :: IO ()
main = hspec $ do
  describe "Prelude.read" $ do
    it "can parse integers" $ do
      read "10" `shouldBe` (10 :: Int)

    it "can parse floating-point numbers" $ do
      read "2.6" `shouldBe` (2.5 :: Float)
