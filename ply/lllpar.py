from llllex import tokens

def p_list_of_ass(p):
    "list_of_ass : list_of_ass assignment"
    if isinstance(p[1], list):
        p[1].append(p[2])
        p[0] = p[1]
    else:
        p[0] = [p[1], p[2]]

def p_list_of_ass_single(p):
    "list_of_ass : assignment"
    p[0] = p[1]

def p_assignment(p):
    "assignment : ID EQUALS expression"
    p[0] = ('ASSIGNMENT', p[1], p[3])

def p_expression_function(p):
    "expression : FN LPAREN expression_list_opt RPAREN LBRACKET list_of_ass RBRACKET"
    p[0] = ("FUNCTION", p[3], p[6])

def p_expression_module(p):
    "expression : MODULE LBRACKET list_of_ass RBRACKET"
    p[0] = ("MODULE", p[3])

def p_expression_func_call(p):
    "expression : ID LPAREN expression_list_opt RPAREN"
    p[0] = ("FUN_CALL", p[1], p[3])

def p_right_eq_case(p):
    "expression : CASE expression LBRACKET list_of_ass RBRACKET"
    p[0] = ("CASE", p[2], p[4])

def p_right_eq_do(p):
    "expression : DO LBRACKET list_of_ass RBRACKET"
    p[0] = ("DO", p[3])

def p_right_eq_union(p):
    "expression : UNION LBRACKET expression_list RBRACKET"
    p[0] = ("DO", p[3])

def p_right_eq_list(p):
    "expression : LSQBRACKET expression_list RSQBRACKET"
    p[0] = ("LIST", p[2])

def p_right_eq_tuple(p):
    "expression : LPAREN expression_list RPAREN" #';'
    p[0] = ("TUPLE", p[2])

def p_expression_dot(p):
    "expression : dot_list"
    # print("p_expression_dot")
    p[0] = p[1]

def p_expression_list_opt(p):
    "expression_list_opt : expression_list"
    p[0] = [1]

def p_expression_list_opt_empty(p):
    "expression_list_opt : empty"
    p[0] = None

def p_expression_list(p):
    "expression_list : expression_list COMMA expression"
    if isinstance(p[1], list):
        p[1].append(p[3])
        p[0] = p[1]
    else:
        p[0] = [p[1], p[3]]

def p_expression_list_single(p):
    "expression_list : expression"
    p[0] = p[1]

def p_dot_list(p):
    "dot_list : dot_list DOT dot_item"
    # print("p_dot_list")
    if isinstance(p[1], list):
        p[1].append(p[3])
        p[0] = p[1]
    else:
        p[0] = [p[1], p[3]]

def p_dot_list_single(p):
    "dot_list : dot_item"
    # print("p_dot_list_single")
    p[0] = ("DOT", p[1])

def p_dot_item_int(p):
    "dot_item : INT"
    # print("INT")
    p[0] = ("INT", p[1])

def p_dot_item_id(p):
    "dot_item : ID"
    # print("ID")
    p[0] = p[1]

def p_error(p):
    print("Parsing error", p.type)
    if not p:
        print("End of File!")
        return

def p_empty(p):
    'empty :'
    pass

import ply.yacc as yacc
parser = yacc.yacc()