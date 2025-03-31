{-# LANGUAGE OverloadedStrings #-}

module Main where

import Data.Binary qualified as BN
import Data.ByteString qualified as B
import Data.ByteString.Builder qualified as BD
import Data.ByteString.Char8 qualified as BC
import Data.ByteString.Lazy qualified as L
import System.Environment (getArgs)
import System.IO

preReplicate :: Int -> a -> [a] -> [a]
preReplicate 0 _ xs = xs
preReplicate n x xs
  | n > 0 = preReplicate (n - 1) x $ x : xs
  | otherwise = error "cannot replicate a negative number of times"

mkWaldo :: BN.Word64 -> Int -> L.ByteString
mkWaldo pos noiseLen =
  let noiseSegment = BD.lazyByteString . L.pack $ preReplicate noiseLen 0xFF [0x00]
      hint = BD.word64BE pos
      padding = BD.lazyByteString . L.pack $ replicate ((fromIntegral pos - (noiseLen + 9))) 0x40
      waldo = BD.byteString "Waldo"
   in BD.toLazyByteString $ hint <> noiseSegment <> padding <> waldo

main :: IO ()
main = do
  args <- getArgs
  case args of
    rawPos : rawNoise : filename : _ -> do
      let pos = read rawPos
      let noiseLen = read rawNoise
      L.writeFile filename $ mkWaldo (fromIntegral pos) noiseLen
    _ -> hPutStrLn stderr "usage: MakeWaldo <pos> <noiseLen> <filename>"
