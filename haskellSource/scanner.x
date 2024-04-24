{
module Main (main) where
}

%wrapper "basic"

$digit = 0-9            -- digits
$alpha = [a-zA-Z]       -- alphabetic characters

tokens :-
  $white+                       ;
  "--".*                        ;
  struct                        { \s -> Struct }
  union                         { \s -> Union }
  fn                            { \s -> Fn }
  var                           { \s -> Var }
  "->"                          { \s -> Arrow }
  true                          { \s -> TokTrue }
  false                         { \s -> TokFalse }
  match                         { \s -> Match }
  $digit+ . $digit+             { \s -> Float (read s) }
  $digit+                       { \s -> Int (read s) }
  [\=\;\,\{\}\(\)]              { \s -> Sym (head s) }
  $alpha [$alpha $digit \_ \']* { \s -> Id s }
--TODO char
--TODO string

{
data Token
  = Struct
  | Var
  | Arrow
  | Union
  | Fn
  | TokTrue
  | TokFalse
  | Match
  | Sym Char
  | Id String
  | Int Int
  | Float Float
  deriving (Eq, Show)

main = do
  print (alexScanTokens "fn main(){x = 5; return(0);}")
}