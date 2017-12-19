module Test.Main where

import Control.Monad.Aff.AVar (AVAR)
import Control.Monad.Eff (kind Effect, Eff)
import Control.Monad.Eff.Console (CONSOLE)
import Data.Either (Either(..))
import Data.Array (fromFoldable)
import Data.List (length, foldl)
import Data.List.Types (List(..), (:))
import Main (Instruction(..), Reg(..), Value(..), Env(..), compile, interpret, parse, eval, exec, init, init0, init1, keep)
import Node.Encoding (Encoding(..))
import Node.FS (FS)
import Node.FS.Aff as FS
import Prelude (Unit, bind, discard, negate, ($), (+), (==))
import Test.Unit (suite, test)
import Test.Unit.Assert as Assert
import Test.Unit.Console (TESTOUTPUT)
import Test.Unit.Main (runTest)
import Partial.Unsafe (unsafePartial)

main :: forall e.                  
  Eff                         
    ( console :: CONSOLE      
    , testOutput :: TESTOUTPUT
    , avar :: AVAR            
    , fs :: FS                
    | e                    
    )                         
    Unit                      
main = runTest do
  suite "sync code" do
    test "arithmetic" do
      Assert.assert "2 + 2 should be 4" $ (2 + 2) == 4
      Assert.assertFalse "2 + 2 shouldn't be 5" $ (2 + 2) == 5
      Assert.equal 4 (2 + 2)
      Assert.expectFailure "2 + 2 shouldn't be 5" $ Assert.equal 5 (2 + 2)
    test "compile set" do
      Assert.equal (Right(Set (Reg "a") (Literal 148))) (compile "set a 148")
      Assert.equal (Right(Set (Reg "z") (Literal (-148)))) (compile "set z -148")
      Assert.equal (Right(Set (Reg "b") (RegLiteral (Reg "c")))) (compile "set b c")
      Assert.equal (Left "set z") (compile "set z")
    test "compile add" do
      Assert.equal (Right(Add (Reg "a") (Literal 148))) (compile "add a 148")
      Assert.equal (Right(Add (Reg "z") (Literal (-148)))) (compile "add z -148")
      Assert.equal (Right(Add (Reg "b") (RegLiteral (Reg "c")))) (compile "add b c")
      Assert.equal (Left "add z") (compile "add z")
    test "compile mul" do
      Assert.equal (Right(Mul (Reg "a") (Literal 148))) (compile "mul a 148")
      Assert.equal (Right(Mul (Reg "z") (Literal (-148)))) (compile "mul z -148")
      Assert.equal (Right(Mul (Reg "b") (RegLiteral (Reg "c")))) (compile "mul b c")
      Assert.equal (Left "mul z") (compile "mul z")
    test "compile mod" do
      Assert.equal (Right(Mod (Reg "a") (Literal 148))) (compile "mod a 148")
      Assert.equal (Right(Mod (Reg "z") (Literal (-148)))) (compile "mod z -148")
      Assert.equal (Right(Mod (Reg "b") (RegLiteral (Reg "c")))) (compile "mod b c")
      Assert.equal (Left "mod z") (compile "mod z")
    test "compile snd" do
      Assert.equal (Right(Snd (Reg "a"))) (compile "snd a")
      Assert.equal (Right(Snd (Reg "z"))) (compile "snd z")
      Assert.equal (Left "snd z 123") (compile "snd z 123")
    test "compile rcv" do
      Assert.equal (Right(Rcv (Reg "a"))) (compile "rcv a")
      Assert.equal (Right(Rcv (Reg "z"))) (compile "rcv z")
      Assert.equal (Left "rcv z 123") (compile "rcv z 123")
    test "compile jgz" do
      Assert.equal (Right(Jgz (Reg "a") (Literal 148))) (compile "jgz a 148")
      Assert.equal (Right(Jgz (Reg "z") (Literal (-148)))) (compile "jgz z -148")
      Assert.equal (Right(Jgz (Reg "b") (RegLiteral (Reg "c")))) (compile "jgz b c")
      Assert.equal (Left "jgz b") (compile "jgz b")
    test "compile unknown" do
      Assert.equal (Left "xxx a 148") (compile "xxx a 148")
      Assert.equal (Left "xxx z -148") (compile "xxx z -148")
      Assert.equal (Left "xxx b c") (compile "xxx b c")
    test "eval" do
      Assert.equal 123 (eval (init0 []) (Literal 123))
      Assert.equal 0 (eval (init0 []) (RegLiteral (Reg "x")))
      Assert.equal 124 (eval (init0 [124]) (RegLiteral (Reg "x")))
    test "exec Set" do
      Assert.equal (init1 [12]) (exec (init []) (Set (Reg "x") (Literal 12)))
      Assert.equal (init1 [12, 12]) (exec (init0 [12]) (Set (Reg "y") (RegLiteral (Reg "x"))))
      Assert.equal (init1 [13]) (exec (init0 [99]) (Set (Reg "x") (Literal 13)))
    test "exec Add" do
      Assert.equal (init1 [12]) (exec (init []) (Add (Reg "x") (Literal 12)))
      Assert.equal (init1 [99]) (exec (init0 [80]) (Add (Reg "x") (Literal 19)))
      Assert.equal (init1 [80, 80]) (exec (init0 [80]) (Add (Reg "y") (RegLiteral (Reg "x"))))
      Assert.equal (init1 [90, 10]) (exec (init0 [80, 10]) (Add (Reg "x") (RegLiteral (Reg "y"))))
    test "exec Mul" do
      Assert.equal (init1 [0]) (exec (init []) (Mul (Reg "x") (Literal 12)))
      Assert.equal (init1 [800]) (exec (init0 [80]) (Mul (Reg "x") (Literal 10)))
      Assert.equal (init1 [80, 0]) (exec (init0 [80]) (Mul (Reg "y") (RegLiteral (Reg "x"))))
      Assert.equal (init1 [800, 10]) (exec (init0 [80, 10]) (Mul (Reg "x") (RegLiteral (Reg "y"))))
    test "exec Mod" do
      Assert.equal (init1 [0]) (exec (init []) (Mod (Reg "x") (Literal 12)))
      Assert.equal (init1 [8]) (exec (init0 [18]) (Mod (Reg "x") (Literal 10)))
      Assert.equal (init1 [80, 0]) (exec (init0 [80]) (Mod (Reg "y") (RegLiteral (Reg "x"))))
      Assert.equal (init1 [8, 10]) (exec (init0 [18, 10]) (Mod (Reg "x") (RegLiteral (Reg "y"))))
    test "exec Snd" do
      Assert.equal (init1 [0,0,0]) (exec (init0 [0,0]) (Snd (Reg "x")))
      Assert.equal (init1 [99,0,99]) (exec (init0 [99,0]) (Snd (Reg "x")))
    test "exec Rcv" do
      Assert.equal (init1 []) (exec (init []) (Rcv (Reg "x")))
      Assert.equal (init1 [1,0,99,99]) (exec (init0 [1,0,99]) (Rcv (Reg "x")))
    test "exec Jgz" do
      Assert.equal (init1 []) (exec (init []) (Jgz (Reg "x") (Literal 12)))
      Assert.equal (init [10, 18]) (exec (init0 [18]) (Jgz (Reg "x") (Literal 10)))
      Assert.equal (init [85, 80, 10]) (exec (init [5,80,10]) (Jgz (Reg "y") (RegLiteral (Reg "x"))))
      Assert.equal (init1 [0, 10]) (exec (init0 [0, 10]) (Jgz (Reg "x") (RegLiteral (Reg "y"))))
    test "keep" do
      Assert.equal (Cons (Snd (Reg "x")) Nil) (keep (Cons (Right (Snd (Reg "x"))) Nil))
      Assert.equal (Cons (Snd (Reg "x")) Nil) (keep (Cons (Left "oops") (Cons (Right (Snd (Reg "x"))) Nil)))
      Assert.equal (Nil) (keep (Cons (Left "oops") Nil))

  suite "AoC 18.1" do
    -- test "with async IO" do
    --   fileContents <- FS.readTextFile UTF8 "input.txt"
    --   Assert.equal "hello here are your file contents\n" fileContents
    test "Read simple input" do
      fileContents <- FS.readTextFile UTF8 "input.txt"
      Assert.equal (init1 []) (unsafePartial (interpret 0 (init0 []) $ fromFoldable $ keep $ parse fileContents))
      Assert.equal 10 (length $ parse fileContents)
      