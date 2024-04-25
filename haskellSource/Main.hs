module Main where

import System.Environment (getArgs)
import Scanner (scannerMain)

main = do
    args <- getArgs
    if length args > 0 then do
        print (args !! 0)
        file <- readFile (args !! 0)
        print file
        print (scannerMain file)
    else
        print "No input file name"