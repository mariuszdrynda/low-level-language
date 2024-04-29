module Main where

import System.Environment (getArgs)
import Scanner (scannerMain)
import Parser

main = do
    args <- getArgs
    if length args > 0 then do
        file <- readFile (args !! 0)
        print file
        let tokens = scannerMain file
        print tokens
        let ast = lllParser tokens
        print ast
    else
        print "No input file name"