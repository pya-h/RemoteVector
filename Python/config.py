SOCKET_HOST = 'localhost'
SOCKET_PORT = 4190
PACKET_SIZE = 1024

def is_number(x: any) -> bool:
    try:
        x = float(x)
        return True
    except:
        return False
    
OPERATORS = {
    'add': '+',
    'sub': '-',
    'mul': '*',
    'dot': '@',
    'abs': '|',
    'div': '/'
}

def contains_operator(params):
    for op in OPERATORS.values():
        if op in params:
            return True
    return False