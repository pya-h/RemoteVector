import socket
import time
from config import *
import threading
from Vectors.vectornd import VectorND


class SocketServer:
    def __init__(self, host = SOCKET_HOST, port = SOCKET_PORT):
        self.port = port
        self.host = host
        self.server = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self.server.bind((host, port))
        self.memory = {}
        
    def start(self):
        self.server.listen()
        print(f'server started listening on {self.host}:{self.port}')
        while True:
            client_socket, ip_address = self.server.accept()
            self.memory[ip_address] = {}
            now = time.ctime()
            print("New client connected to the server on:", now)
            client_socket.send(bytes(f"Welcome! You \'ve connected to Remote Calculator on: {now}", 'utf-8'))
            # receive the client's username
            thread = threading.Thread(target=self.listen, args=(client_socket, ip_address,))
            thread.start()
            # client.socket.close()

    @staticmethod
    def calculate_expression(memory, params):
        result = None
        count = len(params)
        try:
            result = memory[params[0]]
            i = 1
            while i < count:
                op = params[i]
                if i + 1 >= count:
                    raise SyntaxError("Expression is not in correct math form!")
                operand = None
                if is_number(params[i + 1]):
                    operand = float(params[i + 1]) if '.' in params[i + 1] else int(params[i + 1])
                elif params[i + 1] in memory:
                    operand = memory[params[i + 1]]
                elif params[i + 1][0] == OPERATORS['abs'] and params[i + 1][-1] == OPERATORS['abs']:
                    v = params[i + 1][1:-1]
                    if v in memory:
                        if isinstance(v, VectorND):
                            operand = len(memory[v])
                        else:
                            # v is number then | | is abs
                            operand = memory[v] if memory[v] >= 0 else -memory[v]
                    elif is_number(v):
                        v = float(v) if '.' in v else int(v) 
                        operand = v if v >= 0 else -v
                    else:
                        raise ValueError(f"Variable {v} isnt defined!")
                if operand is None:
                    raise ValueError(f"Entity:{params[i + 1]} was not found")
                
                if op == OPERATORS['add']:
                    result += operand
                elif op == OPERATORS['sub']:
                    result -= operand
                elif op == OPERATORS['mul']:
                    result *= operand
                elif op == OPERATORS['dot']:
                    result @= operand
                elif op == OPERATORS['div']:
                    result /= operand
                else:
                    raise SyntaxError(f"Expression contains unsupported operator:{op}")
                i += 2
        except Exception as ex:
            result = ex.__str__()
        return result
    
    def listen(self, client_socket: socket, ip):
        mem = self.memory[ip]
        while True:
            try:
                statement = client_socket.recv(PACKET_SIZE)
                params = statement.decode().split()
                if params[0].lower() == 'exit':
                    self.disconnect_client(client_socket)
                    print("One client disconnected on: ", time.ctime())
                    return

                # if len(params) != 3:
                #    raise ValueError("The statement you\'ve entered is not valid! try again...")
                result = 0
                count = len(params)
                
                if count >= 3 and params[1] == '=':
                    if is_number(params[0]):
                        raise SyntaxError("Variable name can not be a number!")
                    elif params[0][0] == OPERATORS['abs'] and params[0][-1] == OPERATORS['abs']:
                        raise SystemError('| | is an operator, variable name shouldnt be within that!')
                    if count > 3:
                        comps = []
                        rhs = params[2:]
                        if not contains_operator(rhs):
                            # TODO: rewrite this to combine calculations and defining together
                            for x in rhs:
                                if x in mem:
                                    comps.append(mem[x])
                                elif is_number(x):
                                    comps.append(float(x) if '.' in x else int(x))
                                else:
                                    raise ValueError(f"Unknown value:{x} !")
                            mem[params[0]] = VectorND(*comps)
                        else:
                            mem[params[0]] = SocketServer.calculate_expression(memory=mem, params=rhs)
                    else:
                        try:
                            mem[params[0]] = int(params[2])
                        except:
                            mem[params[0]] = float(params[2])
                            
                    result = f"{params[0]} = {mem[params[0]]}"
                else:
                    result = SocketServer.calculate_expression(mem, params)

                client_socket.send(bytes(str(result), 'utf-8'))
            except Exception as e:
                print(e)
                client_socket.send(bytes(e.__str__(), 'utf-8'))

    def disconnect_client(self, client_socket):
        client_socket.send(b'Bye Bye!')
        client_socket.close()

if __name__ == '__main__':
    socket_server = SocketServer()
    socket_server.start()
