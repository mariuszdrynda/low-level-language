reserved = {
    'fn' : 'FN',
    'module' : 'MODULE',
    'case' : 'CASE',
    'do' : 'DO',
    'union' : 'UNION',
}
tokens = ['EQUALS', 'INT', 'ID', 'LBRACKET', 'RBRACKET', 'LPAREN', 'RPAREN', 'COMMA', 'DOT', 'LSQBRACKET', 'RSQBRACKET'] + list(reserved.values())

# t_DOUBLECOLON = r':'
t_COMMA = r','
t_DOT = r'\.'
t_LSQBRACKET = r'\['
t_RSQBRACKET = r'\]'
t_LBRACKET = r'{'
t_RBRACKET = r'}'
t_LPAREN  = r'\('
t_RPAREN  = r'\)'
t_EQUALS  = r'='
t_ignore  = ' \t'
t_ignore_COMMENT = r'\#.*\n'

# digit            = r'([0-9])'
# nondigit         = r'([_A-Za-z])'
# identifier       = r'(' + nondigit + r'(' + digit + r'|' + nondigit + r')*)'    

def t_ID(t):
    r'[a-zA-Z_][a-zA-Z_0-9]*'
    t.type = reserved.get(t.value,'ID')
    return t

def t_INT(t):
    r'-?\d+'
    try:
        t.value = int(t.value)
    except ValueError:
        print("Integer value too large %d", t.value)
        t.value = 0
    return t

def t_newline(t):
     r'\n+'
     t.lexer.lineno += len(t.value)

def find_column(input, token):
    line_start = input.rfind('\n', 0, token.lexpos) + 1
    return (token.lexpos - line_start) + 1

def t_error(t):
    print("Illegal character '%s'" % t.value[0])
    t.lexer.skip(1)

def t_eof(t):
    return None
 
import ply.lex as lex
lexer = lex.lex()