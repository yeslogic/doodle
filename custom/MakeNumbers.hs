module Main where

import Data.Binary qualified as BN
import Data.ByteString qualified as B
import Data.ByteString.Builder qualified as BD
import Data.ByteString.Char8 qualified as BC
import Data.ByteString.Lazy qualified as L
import System.Environment (getArgs)
import System.IO
import Control.Monad.IO.Class (MonadIO)

import System.Random.Stateful (StatefulGen, uniformRM, newIOGenM, newStdGen)

data Rep = U8 | U16 | U32 | U64 | I8 | I16 | I32 | I64
  deriving (Eq, Ord, Show, Read, Enum)

writeRep :: Rep -> Integer -> BD.Builder
writeRep rep val = case rep of
  U8 -> BD.word8 0x00 <> BD.word8 (fromIntegral val)
  U16 -> BD.word8 0x01 <> BD.word16BE (fromIntegral val)
  U32 -> BD.word8 0x02 <> BD.word32BE (fromIntegral val)
  U64 -> BD.word8 0x03 <> BD.word64BE (fromIntegral val)
  I8 -> BD.word8 0x04 <> BD.int8 (fromIntegral val)
  I16 -> BD.word8 0x05 <> BD.int16BE (fromIntegral val)
  I32 -> BD.word8 0x06 <> BD.int32BE (fromIntegral val)
  I64 -> BD.word8 0x07 <> BD.int64BE (fromIntegral val)

rand :: StatefulGen g m => g -> m (Rep, Integer)
rand gen = do
  rep <- toEnum <$> uniformRM (0, 7) gen
  val <- case rep of
    U8 -> uniformRM (0, 255) gen
    U16 -> uniformRM (0, 65535) gen
    U32 -> uniformRM (0, 4294967295) gen
    U64 -> uniformRM (0, 18446744073709551615) gen
    I8 -> uniformRM (-128, 127) gen
    I16 -> uniformRM (-32768, 32767) gen
    I32 -> uniformRM (-2147483648, 2147483647) gen
    I64 -> uniformRM (-9223372036854775808, 9223372036854775807) gen
  return (rep, val)

randListN :: (MonadIO m, StatefulGen g m, m ~ IO) => Int -> g -> m [(Rep, Integer)]
randListN n gen = do
  let go 0 acc = return acc
      go m acc = do
        pair <- rand gen
        go (m - 1) (pair : acc)
  go n []

main :: IO ()
main = do
  args <- getArgs
  case args of
    (fileName:count:_) -> do
      let n = read count :: Integer
      stdGen <- newStdGen
      gen <- newIOGenM stdGen
      values <- randListN (fromIntegral n) gen
      let builders = [writeRep rep val | (rep, val) <- values]
          finalBuilder = mconcat builders
      handle <- openBinaryFile fileName WriteMode
      BD.hPutBuilder handle finalBuilder
      hClose handle
