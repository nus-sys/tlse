# CA_FILE=examples/cert/ca.pem python3 examples/tlsclient.py
from socket import *
from ssl import *
from os import environ

hostname = environ.get('TLS_HOSTNAME', '10.0.1.8:10000')
context = SSLContext(PROTOCOL_TLS_CLIENT)
context.load_verify_locations(environ.get('CA_FILE', './examples/cert/ca.pem'))

with socket(AF_INET, SOCK_STREAM) as sock:
    with context.wrap_socket(sock, server_hostname=hostname) as s:
        s.connect(("127.0.0.1", 8000))
        print(s)
        for i in range(3):
            msg = f"hello{i}"
            s.send(msg.encode())
            reply = s.recv(4 << 10)
            print(reply)
