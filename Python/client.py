import socket
from config import *

class SocketClient:

    def __init__(self, host = SOCKET_HOST, port = SOCKET_PORT):
        self.socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self.socket.connect((host, port))
        # wait for server answer to the connection
        response = self.socket.recv(PACKET_SIZE)
        print(response.decode())

    def communicate(self):
        while True:
            statement = input('> ')
            # send input to the server
            self.socket.send(bytes(statement, 'utf-8'))
            # wait for the server answer
            response = self.socket.recv(PACKET_SIZE)
            print(response.decode())
            if statement.lower() == 'exit':
                self.socket.close()
                return

if __name__ == '__main__':
    client = SocketClient()
    client.communicate()
