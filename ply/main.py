from lllpar import parser
import os
import sys

def main():
    test()
    #name = sys.argv[1].split(".")[0]
    #f = open(sys.argv[1], "r")
    #data = f.read()
    #f.close()
    # ast = parser.parse(data)
    # if ast:
    #     print(ast)
    #     print("AST generated successfully")

def test():
    data = '''
stdio = include(std.stdio)
main = module{
    main = fn(){
        io = printf(1)
    }
}
'''
    ast = parser.parse(data)
    print(ast)
    if ast:
        print("TEST SUCCEED")
    else:
        print("TEST FAILED")

main()