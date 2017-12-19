module Main where

import Prelude

import Control.Monad.Eff (Eff)
import Control.Monad.Eff.Console (CONSOLE, log)
import Data.Array (toUnfoldable, cons, index, length)
import Data.Either (Either(Right, Left))
import Data.Generic (class Generic, gShow)
import Data.Int (fromString)
import Data.List (filter)
import Data.List.Types (List(..), (:))
import Data.Map (Map, lookup, showTree, insert, empty, singleton, member)
import Data.Maybe (fromMaybe)
import Data.String as String
import Debug.Trace

newtype Reg = Reg String
derive instance eqReg :: Eq Reg
derive instance ordReg :: Ord Reg
derive instance genericReg :: Generic Reg

data Value = Literal Int | RegLiteral Reg | Unknown String
derive instance eqV :: Eq Value
derive instance ordV :: Ord Value
derive instance genericV :: Generic Value
instance showValue :: Show Value where
    show = gShow

data Instruction = Snd Reg --plays a sound with a frequency equal to the value of X.
    | Set Reg Value --sets register X to the value of Y.
    | Add Reg Value --increases register X by the value of Y.
    | Mul Reg Value --sets register X to the result of multiplying the value contained in register X by the value of Y.
    | Mod Reg Value --sets register X to the remainder of dividing the value contained in register X by the value of Y (that is, it sets X to the result of X modulo Y).
    | Rcv Reg --recovers the frequency of the last sound played, but only when the value of X is not zero. (If it is zero, the command does nothing.)
    | Jgz Reg Value --jumps with an offset of the value of Y, but only if the value of X is greater than zero. (An offset of 2 skips the next instruction, an offset of -1 jumps to the previous instruction, and so on.)
    | UnknownInst String String String -- Unknown action

data Env = Env Int (Map String Int)

init0 :: Array Int -> Env 
init0 l = init (cons 0 l)
init1 :: Array Int -> Env 
init1 l = init (cons 1 l)

init :: Array Int -> Env
init [] = Env 0 (empty)
init [pc] = Env pc (empty)
init [pc,x] = Env pc (singleton "x" x)
init [pc,x,y] = Env pc (insert "y" y (singleton "x" x))
init [pc,x,y,snd] = Env pc (insert "snd" snd (insert "y" y (singleton "x" x)))
init [pc,x,y,snd,rcv] = Env pc (insert "rcv" rcv (insert "snd" snd (insert "y" y (singleton "x" x))))
init l = init []

derive instance eqEnv :: Eq Env
instance showEnv :: Show Env where
    show (Env pc e) = "PC(" <> show pc <> "): " <> showTree e

derive instance genericInstruction :: Generic Instruction
derive instance eqInstruction :: Eq Instruction
derive instance ordInstruction :: Ord Instruction
instance showInstruction :: Show Instruction where
    show = gShow

parseValue :: String -> Value
parseValue s = fromMaybe (RegLiteral (Reg s)) (Literal <$> fromString s)

triple :: (Reg -> Value -> Instruction) -> String -> String -> Instruction
triple f r v = f (Reg r) (parseValue v)

eval :: Env -> Value -> Int
eval e (Literal v) = v
eval (Env pc e) (RegLiteral (Reg r)) = fromMaybe 0 $ lookup r e
eval e _ = 0
evalReg :: Env -> String -> Int
evalReg (Env pc e) r = fromMaybe 0 $ lookup r e

exec :: Env -> Instruction -> Env
exec e@(Env pc m) (Set (Reg r) v) = Env (pc + 1) $ insert r (eval e v) m 
exec e@(Env pc m) (Add (Reg r) v) = Env (pc + 1) $ insert r ((evalReg e r) + (eval e v)) m
exec e@(Env pc m) (Mul (Reg r) v) = Env (pc + 1) $ insert r ((evalReg e r) * (eval e v)) m
exec e@(Env pc m) (Mod (Reg r) v) = Env (pc + 1) $ insert r ((evalReg e r) `mod` (eval e v)) m
exec e@(Env pc m) (Snd (Reg r)) = Env (pc + 1) $ insert "snd" (evalReg e r) m
exec e@(Env pc m) (Rcv (Reg r)) | v <- (evalReg e r), v > 0 = Env (pc + 1) $ insert "rcv" (evalReg e "snd") m
exec e@(Env pc m) (Rcv (Reg r)) = Env (pc + 1) m
exec e@(Env pc m) (Jgz (Reg r) v) | rv <- (evalReg e r), rv > 0 = Env (pc + (eval e v)) m
exec e@(Env pc m) (Jgz (Reg r) v) = Env (pc + 1) m
exec e@(Env pc m) (UnknownInst i j k) = trace "Unknown inst" \_ -> Env (pc + 1) m
--exec (Env pc m) i = Env (pc + 1) m

interpret :: Partial => Int -> Env -> Array Instruction -> Env
interpret i e@(Env pc m) code | (length code) > pc && pc >= 0 && not (member "rcv") m && i < 10000 = interpret (i+1) (fromMaybe e $ exec e <$> (index code pc)) code
interpret i e@(Env pc m) code = Env pc (insert ("done" <> show i) 1 m)

instruction :: Array String -> Either String Instruction
instruction ["set",r,v] = Right $ triple Set r v
instruction ["add",r,v] = Right $ triple Add r v
instruction ["mul",r,v] = Right $ triple Mul r v
instruction ["mod",r,v] = Right $ triple Mod r v
instruction ["snd",r] = Right $ Snd $ Reg r
instruction ["rcv",r] = Right $ Rcv $ Reg r
instruction ["jgz",r,v] = Right $ triple Jgz r v
instruction [i,r,v] = Left $ i <> " " <> r <> " " <> v
instruction [i,r] = Left $ i <> " " <> r
instruction l = Left $ show l

keep :: List (Either String Instruction) -> List Instruction
keep ((Left s) : t) = keep(t)
keep ((Right i) : t) = i : keep(t)
keep Nil = Nil

compile :: String -> Either String Instruction
compile = instruction <<< splitAt " "

splitAt :: String -> String -> Array String
splitAt delim = String.split (String.Pattern delim)

parse :: String -> List (Either String Instruction)
parse s = map compile $ filter nonEmpty $ toUnfoldable $ map String.trim $ splitAt "\n" s
    where nonEmpty x = ((String.length) x) > 0

main :: forall e. Eff (console :: CONSOLE | e) Unit
main = do
  log "Hello sailor!"