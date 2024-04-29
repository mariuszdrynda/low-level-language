{
module Parser where
import Scanner
}

%name lllParser
%tokentype { Token }
%error { parseError }

%token
    struct { TokStruct }
    -- var { TokVar }
    arrow { TokArrow }
    -- union { TokUnion }
    fn { TokFn }
    true { TokTrue }
    false { TokFalse }
    match { TokMatch }
    id { TokId $$ }
    int { TokInt $$ }
    float { TokFloat $$ }
    '=' { TokEq }
    ':' { TokDC }
    ';' { TokSemicolon }
    ',' { TokComma }
    '{' { TokLB }
    '}' { TokRB }
    '(' { TokenLParen }
    ')' { TokenRParen }

%%

Module :: { [AST] }
    : Module Global { $2 : $1 }
    | Global { [ $1 ] }

Global : Function { $1 }
    | GlobalAssignment { $1 }

GlobalAssignment : Id '=' RightGlobalAssignment ';' { ASS $1 $3 }

RightGlobalAssignment : Literal { $1 }
    | Struct { $1 }
    -- | Union { $1 }

Struct : struct id '{' ArgumentsOrEmpty '}' { STRUCT $2 $4 }

-- Union : "union" '{' ArgumentsOrEmpty '}' { UNION $2 $4 }

Function : fn id '(' ArgumentsOrEmpty ')' Type '{' Statements '}' { FUNC $2 $6 $4 $8 }

ArgumentsOrEmpty : ArgumentsWithType { $1 }
    | {- empty -}           { [] }

ArgumentsWithType : ArgumentsWithType ',' AgrumentWithType { $3 : $1 } -- array
    | AgrumentWithType { [ $1 ] }

AgrumentWithType : id ':' Type { ID $3 $1 }

Statements : Statements Statement { $2 : $1 } -- array
    | Statement { [ $1 ] }

Statement : Assignment ';' { $1 }
    | FunctionCall ';' { $1 }
    | Match { $1 }

Match : match Argument '{' CaseBody '}' { MATCH $2 $4 }

Assignment : Id '=' Argument { ASS $1 $3 }

CaseBody : CaseBody Case { $2 : $1 } -- array
    | Case { [ $1 ] }

Case : Argument arrow '{' Statements '}' { CASE $1 $4 }

FunctionCall : id '(' FCArgumentsOrEmpty ')' { CALL UNKNOWN $1 $3 }

FCArgumentsOrEmpty : Arguments { $1 }
    | {- empty -}           { [] }

Arguments : Arguments ',' Argument { $3 : $1 } -- array
    | Argument { [ $1 ] }

Argument : FunctionCall { $1 }
    | Literal { $1 }
    | Match { $1 }

Literal :: { AST }
    : true { BOOL_LIT BOOL True }
    | false { BOOL_LIT BOOL False }
    | int { INT UNKNOWN $1 }
    | float { FLOAT_LIT UNKNOWN $1 }
    -- | Char {}
    -- | Str {}
    | Id { $1 }

Id : id { ID UNKNOWN $1 }

Type : id { TYPE_NAME $1 }

{

parseError :: [Token] -> a
parseError _ = error "Parse error"

data AST = MODULE [AST]
    | STRUCT String [AST]
    | DECL AST AST
    | ASS AST AST
    | REG String AST -- assign to register
    | CALL TYPE String [AST]
    | FUNC String TYPE [AST] [AST]
    | MATCH AST [AST]
    | CASE AST [AST]
    | ID TYPE String
    | INT TYPE Int
    | BOOL_LIT TYPE Bool
    | FLOAT_LIT TYPE Float
    -- | CHAR_LIT Char
    -- | STRING_LIT String
    -- | UNION String [AST]
    deriving (Show)

data TYPE = I32
    | F32
    | PTR TYPE
    | STRING
    | ARRAY Int TYPE
    | STRUCT_TYPE String
    | FN [TYPE] TYPE
    | BOOL
    | FLOW
    | UNKNOWN
    | TYPE_NAME String
    deriving (Show)

}