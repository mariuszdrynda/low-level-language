import qualified Data.Map as Map

-- global functions
i32 = "I32"
set = "Ptr.setElem"
return = "return"

data AST = STRUCT String [AST]
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
    | STR String
    | FN [TYPE] TYPE
    | BOOL
    | FLOW
    | UNKNOWN
    deriving (Show)

-- Hehe = struct{a : I32, b : I32};
-- fn main() -> I32 {
-- 	arr = Array(5, Hehe);
-- 	tmp = Ptr.getElem(arr, 3); # arr[3].b = 12;
-- 	Ptr.setElem(tmp, b, 12);
--  return(0);s
-- }
-- RESULT
-- %struct.Hehe = type { i32, i32 }
-- define dso_local i32 @main() {
--   %arr = alloca [5 x %struct.Hehe]
--   %tmp = getelementptr inbounds [5 x %struct.Hehe], ptr %arr, i64 0, i64 3
--   %3 = getelementptr inbounds %struct.Hehe, ptr %tmp, i32 0, i32 1
--   store i32 12, ptr %3
--   ret i32 0
-- }
test_1 = do
    print "test1 ast0"
    print ast0
    print "test1 ast1"
    print ast1
    print "test1 result"
    print result
    writeFile file result
    where
    ast0 = [STRUCT "Hehe" [ID I32 "a", ID I32 "b"],
        FUNC "main" I32 [] [
            ASS (ID (ARRAY 5 (STR "Hehe")) "arr") (CALL (FN [I32, STR "Hehe"] (ARRAY 5 (STR "Hehe"))) "Array" [INT I32 5, ID (STR "Hehe") "Hehe"]),
            ASS (ID (PTR (STR "Hehe")) "tmp") (CALL UNKNOWN "Ptr.getElem" [ID (ARRAY 5 (STR "Hehe")) "arr", INT I32 3]),
            CALL UNKNOWN "set" [ID (PTR (STR "Hehe")) "tmp", ID STRING "b", INT I32 12],
            CALL UNKNOWN "return" [INT I32 0]]]
    ast1 = map add_struct_dot_to_all_Struct_names ast0
    result = generate_llvm_from_ast_list ast1
    file = "result.txt"

-- fn factorial(n: I32) I32 {
-- 	return(match eq(n, 0){
-- 		True -> { 1; }
-- 		False -> { mul(n, factorial(sub(n, 1))); }
-- 	})
-- }
-- RESULT
-- define i32 @factorial(i32) {
-- entry:
--   %eq = icmp eq i32 %0, 0
--   br i1 %eq, label %then, label %else
-- then:
--   ret i32 1
-- else:
--   %sub = sub i32 %0, 1
--   %2 = call i32 @factorial(i32 %sub)
--   %mult = mul i32 %0, %2
--   ret i32 %mult
-- }
test_2 = do
    print "test2 ast0"
    print ast0
    print "test2 ast1"
    print ast1
    --print "test2 ast2"
    --print ast2
    where
    ast0 = [FUNC "factorial" I32 [ID I32 "n"] [
        CALL UNKNOWN "return" [
            MATCH (CALL UNKNOWN "eq" [ID UNKNOWN "n", INT I32 0]) [
                CASE (BOOL_LIT BOOL True) [INT I32 1],
                CASE (BOOL_LIT BOOL False) [CALL UNKNOWN "mul" [ID UNKNOWN "n", CALL UNKNOWN "factorial" [CALL UNKNOWN "sub" [ID UNKNOWN "n", INT I32 1]]]]]]]]
    ast1 = return_propagation ast0

-- SSA FORM
-- fn factorial(n: I32) I32 {
--   tmp0 = eq(n, 0);
--   tmp1: I32 = undefined;
--   match tmp0{
-- 		True -> { tmp1 = 1; }
-- 		False -> { 
--        tmp2 = sub(n, 1);
--        tmp3 = factorial(tmp2);
--        tmp1 = mul(n, tmp3);
--    }
-- 	}
-- 	return(tmp1);
-- }
test_3 = do
    print "test3 ast0"
    print ast0
    print "result"
    print result
    writeFile file result
    where
    ast0 = [FUNC "factorial" I32 [ID I32 "n"] [
        REG "tmp0" (CALL UNKNOWN "eq" [ID I32 "n", INT I32 0]),
        ASS (ID I32 "tmp1") (ID I32 "undefined"),
        MATCH (ID BOOL "tmp0") [
            CASE (BOOL_LIT BOOL True) [ASS (ID I32 "tmp1") (INT I32 1)],
            CASE (BOOL_LIT BOOL False)
                    [REG "tmp2" (CALL (FN [I32, I32] I32) "sub" [ID I32 "n", INT I32 1]),
                    REG "tmp3" (CALL (FN [I32] I32) "factorial" [ID I32 "tmp2"]),
                    ASS (ID I32 "tmp1") (CALL (FN [I32, I32] I32) "mul" [ID I32 "n", ID I32 "tmp2"])]],
        CALL UNKNOWN "return" [(ID I32 "tmp1")]]]
    ast1 = return_propagation ast0
    result = generate_llvm_from_ast_list ast1
    file = "result.ll"

-- # LLL
-- a = I32(5);
-- b = Ptr.addr(a); #:Ptr(I32)
-- set(b, 7);
-- ; LL
-- %1 = alloca i32
-- store i32 5, ptr %1
-- %2 = alloca ptr
-- store ptr %1, ptr %2
-- %3 = load ptr, ptr %2
-- store i32 7, ptr %3
test_4 = do
    print "test4"
    print "ast0"
    print ast0
    print "ast3"
    print ast3
    print "result"
    print result
    writeFile file result
    where
    ast0 = [DECL (ID I32 "a") (CALL UNKNOWN "I32" [(INT UNKNOWN 5)]),
        DECL (ID I32 "b") (CALL UNKNOWN "Ptr.addr" [(ID I32 "a")]),
        (CALL UNKNOWN "set" [(ID I32 "b"), (INT I32 7)])]
    ast1 = map comptime_casting ast0 --TODO TEST
    ast2 = return_propagation ast1
    ast3 = memory_vars_to_load_store ast2
    result = generate_llvm_from_ast_list ast3
    file = "result.ll"

comptime_casting :: AST -> AST
comptime_casting (CALL _ i32 [(INT _ v)]) = (INT I32 v)
comptime_casting (CALL t n a) = CALL t n (map comptime_casting a)
comptime_casting (DECL l r) = DECL l (comptime_casting r)
comptime_casting (ASS l r) = ASS l (comptime_casting r)
comptime_casting (REG l r) = REG l (comptime_casting r)
comptime_casting (FUNC n t a stms) = FUNC n t a (map comptime_casting stms)
comptime_casting (MATCH e c) = MATCH (comptime_casting e)  (map comptime_casting c) 
comptime_casting (CASE l r) = CASE l (map comptime_casting r)
comptime_casting x = x

memory_vars_to_load_store :: [AST] -> [AST]
memory_vars_to_load_store [] = []
-- memory_vars_to_load_store ((ASS (ID t n) x) : xs) =
memory_vars_to_load_store (x : xs) = x : memory_vars_to_load_store xs

return_propagation :: [AST] -> [AST]
return_propagation [] = []
return_propagation ((FUNC n t a stms) : xs) = (FUNC n t a (return_propagation stms)) : return_propagation xs
return_propagation ((CALL _ return [MATCH e cases]) : xs) = MATCH e (in_cases (return_propagation cases)) : return_propagation xs
    where
    in_cases :: [AST] -> [AST]
    in_cases [] = []
    in_cases ((CASE l r) : xs) = (CASE l (last_case_to_return r)) : in_cases xs
    last_case_to_return :: [AST] -> [AST]
    last_case_to_return [] = []
    last_case_to_return [x] = [CALL UNKNOWN return [x]]
    last_case_to_return (x : xs) = x : last_case_to_return xs
return_propagation ((CASE l r) : xs) = (CASE l (return_propagation r)) : return_propagation xs
return_propagation (x : xs) = x : return_propagation xs

generate_type :: TYPE -> String
generate_type BOOL = "i1"
generate_type I32 = "i32"
generate_type _ = "TODO" --TODO

generate_op_args :: [AST] -> String
generate_op_args [] = ""
generate_op_args [arg] = generate_op_arg arg
generate_op_args (arg : xs) = generate_op_arg arg ++ ", " ++ generate_op_args xs

generate_op_arg :: AST -> String
generate_op_arg (ID _ n) = "%" ++ n
generate_op_arg (INT _ v) = show v
generate_op_arg (FLOAT_LIT _ v) = show v

generate_call_args :: [AST] -> String
generate_call_args [] = ""
generate_call_args [arg] = generate_call_arg arg
generate_call_args (arg : xs) = generate_call_arg arg ++ ", " ++ generate_call_args xs

generate_call_arg :: AST -> String
generate_call_arg (ID t n) = generate_type t ++ " %" ++ n
generate_call_arg x = "" --TODO

generate_llvm :: AST -> String
generate_llvm (STRUCT name args) = "%" ++ name ++ " = type {" ++ generate_struct_fields args ++ "}" ++ "\n"
    where
    generate_struct_fields :: [AST] -> String
    generate_struct_fields [] = "" --TODO
    generate_struct_fields ((ID t _) : []) = generate_type t
    generate_struct_fields ((ID t _) : xs) = generate_type t ++ ", " ++ generate_struct_fields xs
generate_llvm (FUNC name t args stms) = "define dso_local " ++ generate_type t ++ " @" ++ name ++ "(" ++ generate_args args ++ ") {\n" ++ generate_llvm_from_ast_list stms ++ "}\n"
    where
    generate_args :: [AST] -> String
    generate_args [] = ""
    generate_args [(ID t0 n0)] = generate_type t0 ++ " %" ++ n0
    generate_args [(ID t0 n0), (ID t1 n1)] = generate_type t0 ++ " %" ++ n0 ++ ", " ++ generate_type t1 ++ " %" ++ n1
    generate_args ((ID t0 n0) : xs) = generate_type t0 ++ " %" ++ n0 ++ ", " ++ generate_args xs
generate_llvm (ASS (ID t0 name) (INT t1 val)) = "store " ++ generate_type t0 ++ " " ++ show val ++ ", ptr %" ++ name ++ "\n"
generate_llvm (ASS (ID t name) (ID _ "undefined")) = "%" ++ name ++ " = alloca " ++ generate_type t ++ "\n"
generate_llvm (ASS (ID _ left) (CALL _ "Array" [INT I32 size, ID (STR _) type_name])) = "%" ++ left ++ " = alloca [" ++ show size ++ " x %" ++type_name ++ "]\n"
generate_llvm (ASS (ID _ left) (CALL (FN _ t) "mul" args)) = "%" ++ tmp ++ " = mul " ++ generate_type t ++ " " ++ generate_op_args args ++ "\n" ++ "store " ++ generate_type t ++ " %" ++ tmp ++ ", ptr %" ++ left ++ "\n"
    where
    tmp = "tmp01"
generate_llvm (ASS (ID _ left) (CALL _ "Ptr.getElem" [ID (ARRAY size_of_array (STR struct_name)) array_name, INT I32 index])) =
    "%" ++ left ++ " = getelementptr inbounds [" ++ show size_of_array ++" x %" ++ struct_name ++ "], ptr %" ++ array_name ++ ", i64 0, i64 " ++ show index ++ "\n"
generate_llvm (ASS left right) = "" --TODO
generate_llvm (REG left (CALL _ "eq" [ID t name, INT _ value])) = "%" ++ left ++ " = icmp eq " ++ generate_type t ++ " %" ++ name ++ ", " ++ show value ++ "\n"
generate_llvm (REG left (CALL (FN _ t) "sub" args)) = "%" ++ left ++ " = sub " ++ generate_type t ++ " " ++ generate_op_args args ++ "\n"
generate_llvm (REG left (CALL (FN _ t) fn_name args)) = "%" ++ left ++ " = call " ++ generate_type t ++ " @" ++ fn_name ++ "(" ++ generate_call_args args ++")\n"
generate_llvm (CALL _ set [ID (PTR (STR str_name)) name_arg_0, ID STRING field, INT t value]) = "%" ++ tmp ++ " = getelementptr inbounds %" ++ str_name ++ ", ptr %" ++ name_arg_0 ++ ", i32 0, i32 " ++ show field_to_number ++ "\nstore " ++ show (generate_type t) ++ " " ++ show value ++ ", ptr %" ++ tmp ++ "\n"
    where
        field_to_number = 1 -- TODO
        tmp = "3" -- TODO
generate_llvm (CALL _ return [INT t value]) = "ret " ++ generate_type t ++ " " ++ show value ++ "\n"
generate_llvm (CALL _ return [ID t name]) = "ret " ++ generate_type t ++ " %" ++ name ++ "\n"
generate_llvm (CALL _ name args) = "" --TODO
generate_llvm (MATCH (ID t name) cases) = "switch " ++ generate_type t ++ " %" ++ name ++ ", label %" ++ show last
    ++ " [" ++ generate_cases_left 0 cases ++ "]\n" ++ generate_cases_right 0 cases
    where
    generate_cases_left :: Int -> [AST] -> String
    generate_cases_left _ [] = ""
    generate_cases_left n [CASE (BOOL_LIT BOOL True) _] = "i1 1, label %" ++ show n
    generate_cases_left n [CASE (BOOL_LIT BOOL False) _] = "i1 0, label %" ++ show n
    generate_cases_left n ((CASE (BOOL_LIT BOOL True) _) : xs) = "i1 1, label %" ++ show n ++ " " ++ generate_cases_left (n+1) xs
    generate_cases_left n ((CASE (BOOL_LIT BOOL False) _) : xs) = "i1 0, label %" ++ show n ++ " " ++ generate_cases_left (n+1) xs
    generate_cases_right :: Int -> [AST] -> String
    generate_cases_right n [CASE _ stms] = show n ++ ":\n" ++ generate_llvm_from_ast_list stms ++ "br label %" ++ show last ++ "\n" ++ show last ++ ":\n"
    generate_cases_right n ((CASE _ stms) : xs) = show n ++ ":\n" ++ generate_llvm_from_ast_list stms ++ "br label %" ++ show last ++ "\n" ++ generate_cases_right (n+1) xs
    last = length cases -- TODO
generate_llvm x = "" --TODO

generate_llvm_from_ast_list :: [AST] -> String
generate_llvm_from_ast_list (x : xs) = generate_llvm x ++ generate_llvm_from_ast_list xs
generate_llvm_from_ast_list [] = ""

-- llvm steps
add_struct_dot_to_all_Struct_names_type :: TYPE -> TYPE
add_struct_dot_to_all_Struct_names_type (PTR x) = add_struct_dot_to_all_Struct_names_type x
add_struct_dot_to_all_Struct_names_type (ARRAY x y) = ARRAY x (add_struct_dot_to_all_Struct_names_type y)
add_struct_dot_to_all_Struct_names_type (STR name) = STR ("struct." ++ name)
add_struct_dot_to_all_Struct_names_type x = x

-- types has to be known
add_struct_dot_to_all_Struct_names :: AST -> AST
add_struct_dot_to_all_Struct_names (STRUCT name args) = STRUCT ("struct." ++ name) args
add_struct_dot_to_all_Struct_names (ASS l r) = ASS (add_struct_dot_to_all_Struct_names l) (add_struct_dot_to_all_Struct_names r)
add_struct_dot_to_all_Struct_names (ID (STR name) x) = ID (STR ("struct." ++ name)) ("struct." ++ x)
add_struct_dot_to_all_Struct_names (ID t x) = ID (add_struct_dot_to_all_Struct_names_type t) x
add_struct_dot_to_all_Struct_names (CALL t n stms) = CALL t n (map add_struct_dot_to_all_Struct_names stms)
add_struct_dot_to_all_Struct_names (FUNC n t args stms) = FUNC n (add_struct_dot_to_all_Struct_names_type t) (map add_struct_dot_to_all_Struct_names args) (map add_struct_dot_to_all_Struct_names stms)
add_struct_dot_to_all_Struct_names (MATCH x stms) = MATCH (add_struct_dot_to_all_Struct_names x) (map add_struct_dot_to_all_Struct_names stms)
add_struct_dot_to_all_Struct_names x = x

all_types :: AST -> AST
-- all_types (ASS (ID UNKNOWN name) r) = ASS (ID ? name) r
all_types x = x
--     | ASS AST AST
--     | ID TYPE String
--     | CALL TYPE String [AST]
--     | INT TYPE Int
--     | BOOL_LIT TYPE Bool
--     | FUNC String TYPE [AST] [AST]
--     | MATCH AST [AST]
--     | CASE AST [AST]

add_fn_args_to_known_types :: AST -> Map.Map String TYPE -> Map.Map String TYPE
add_fn_args_to_known_types (FUNC _ _ args _) list_of_vars = add_fn_args_to_known_types_helper args list_of_vars
    where
    add_fn_args_to_known_types_helper :: [AST] -> Map.Map String TYPE -> Map.Map String TYPE
    add_fn_args_to_known_types_helper [] list_of_vars = list_of_vars
    add_fn_args_to_known_types_helper (ID t n : xs) list_of_vars = add_fn_args_to_known_types_helper xs (Map.insert n t list_of_vars)
    add_fn_args_to_known_types_helper _ list_of_vars = list_of_vars

-- ssa_form :: [AST] -> [AST]
-- ssa_form [] = []
-- ssa_form ((STRUCT n a) : xs) = STRUCT n a : ssa_form xs
-- ssa_form ((FUNC n t arg stms) : xs) = FUNC n t arg (ssa_form_in_fn stms) : ssa_form xs
-- ssa_form ((ASS l r) : xs) = ASS l r : ssa_form xs --global assignment

-- ssa_form_in_fn :: AST -> [AST]
-- ssa_form_in_fn (CALL t name [MATCH expr cases]) = if is_ssa_form expr 
--     then [ASS (ID t temp) [MATCH expr cases], CALL t name [(ID t temp)]]
--     else ssa_form_in_fn expr [ASS (ID t temp) [MATCH expr cases], CALL t name [(ID t temp)]]
--     where
--     t = I32 --todo
--     temp = "tmp" --todo
-- ssa_form_in_fn (CALL t name x) = if is_ssa_form x then [CALL t name x] else ssa_form_in_fn x : [] --todo

-- is_ssa_form :: AST -> Bool
-- is_ssa_form (CALL _ "return" X) = --TODO
-- is_ssa_form (ASS _ r) = --TODO
-- is_ssa_form (CALL _ _ (arg : args)) = --TODO
-- is_ssa_form (STRUCT _ _) = True
-- is_ssa_form (ID _ _) = True
-- is_ssa_form (INT _ _) = True
-- is_ssa_form (BOOL_LIT _ _) = True
-- is_ssa_form (FUNC _ _ _ _) = True
-- is_ssa_form (MATCH expr cases) = 
-- is_ssa_form (CASE AST [AST])

-- data LLVM = LLVM_ALLOCA String TYPE
--     | LLVM_STORE LLVM_VAL LLVM_VAL
--     | LLVM_LOAD String TYPE LLVM_VAL
--     | LLVM_SWITCH TYPE String LLVM_LABEL [(LLVM_VAL, LLVM_LABEL)]
--     | LLVM_FN TYPE String [LLVM]
--     | LLVM_GETELEMENTPTR String TYPE LLVM_PTR [Int]
--     | LLVM_RET LLVM_VAL
--     | LLVM_GLOBAL LLVM_VAL
--     | LLVM_GOTO String
--     | LLVM_LABEL String
--     | LLVM_BR LLVM_LABEL
--     | LLVM_CALL TYPE String [LLVM_VAL]
--     | LLVM_DECLARE TYPE String [LLVM_VAL]
--     | LLVM_ICMP_NE LLVM_VAL LLVM_VAL
--     | LLVM_STRUCT String [TYPE]

-- data LLVM_VAL = LLVM_GLOBAL_ID TYPE String
--     | LLVM_LOCAL_ID TYPE String
--     | LLVM_INT Int
--     | LLVM_STRING String
--     | LLVM_GLOBAL_ARRAY String TYPE [LLVM_VAL]

main :: IO ()
main = do
    test_4
    --print (add_fn_args_to_known_types (FUNC "factorial" I32 [ID I32 "n", ID I32 "b"] []) (Map.fromList []))
    print "end"