{-# LANGUAGE OverloadedStrings #-}

module Main where

import Data.Binary qualified as BN
import Data.ByteString qualified as B
import Data.ByteString.Builder qualified as BD
import Data.ByteString.Char8 qualified as BC
import Data.ByteString.Lazy qualified as L
import System.Environment (getArgs)
import Data.List (group)
import Data.Char (ord)

import Data.Monoid qualified as M

import System.IO


data Style = OldStyle | NewStyle
  deriving (Eq, Ord, Show, Read, Enum)

styleToTag :: Style -> BD.Builder
styleToTag OldStyle = BD.word8 0x00
styleToTag NewStyle = BD.word8 0x01


rlEncode :: [[Char]] -> BD.Builder
rlEncode [] = mempty
rlEncode (run:rest) =
  if length run >= 256
    then
      let (run', overrun) = splitAt 255 run
       in rlEncode' run' <> rlEncode (overrun:rest)
    else
      rlEncode' run <> rlEncode rest
  where
    rlEncode' :: [Char] -> BD.Builder
    rlEncode' [] = mempty
    rlEncode' cs@(c:_) = let n = length cs in BD.word8 (fromIntegral n) <> BD.word8 (fromIntegral $ ord c)


mkRle :: Style -> String -> L.ByteString
mkRle style buf =
  let tag = styleToTag style in
    BD.toLazyByteString $ tag <> rlEncode (group buf)


main :: IO ()
main = do
  args <- getArgs
  case args of
    filename : rawStyle : input : _ -> do
      let style = case rawStyle of
            "old" -> OldStyle
            "new" -> NewStyle
            _ -> error "unknown style"
      L.writeFile filename $ mkRle style input
    _ -> hPutStrLn stderr "usage: MakeRLE <filename> <style> <input>"
