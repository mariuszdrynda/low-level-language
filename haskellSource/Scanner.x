{
module Scanner where
}

%wrapper "basic"

$digit = 0-9            -- digits
$alpha = [a-zA-Z]       -- alphabetic characters

tokens :-
  $white+                       ;
  "--".*                        ;
  struct                        { \s -> TokStruct }
  union                         { \s -> TokUnion }
  fn                            { \s -> TokFn }
  var                           { \s -> TokVar }
  "->"                          { \s -> TokArrow }
  true                          { \s -> TokTrue }
  false                         { \s -> TokFalse }
  match                         { \s -> TokMatch }
  $digit+ . $digit+             { \s -> TokFloat (read s) }
  $digit+                       { \s -> TokInt (read s) }
  =                             { \s -> TokEq }
  \;                            { \s -> TokSemicolon }
  :                             { \s -> TokDC }
  \,                            { \s -> TokComma }
  \{                            { \s -> TokLB }
  \}                            { \s -> TokRB }
  \(                            { \s -> TokenLParen }
  \)                            { \s -> TokenRParen }
  $alpha [$alpha $digit \_ \']* { \s -> TokId s }
--TODO char
--TODO string

{
data Token
  = TokStruct
  | TokEq
  | TokSemicolon
  | TokComma
  | TokDC
  | TokLB
  | TokRB
  | TokenLParen
  | TokenRParen
  | TokVar
  | TokArrow
  | TokUnion
  | TokFn
  | TokTrue
  | TokFalse
  | TokMatch
  | TokSym Char
  | TokId String
  | TokInt Int
  | TokFloat Float
  deriving (Eq, Show)

scannerMain x = do
  alexScanTokens x
}