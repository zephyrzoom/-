import socket
import subprocess
ip_port = ('127.0.0.1', 9999)

s = socket.socket()

s.bind(ip_port)

s.listen(5)


while True:
    conn, addr = s.accept()
    while True:
        try:
            recv_data = conn.recv(1024)
            if recv_data == 0:
                break
            p = subprocess.Popen(
                str(recv_data, encoding='utf8'), shell=True, stdout=subprocess.PIPE)
            res = p.stdout.read()
            if len(res) == 0:
                send_data = 'cmd ERROR'
                send_data = bytes(send_data, encoding='utf8')
            else:
                send_data = res
            ready_tag = 'Ready|%s' % len(send_data)
            conn.send(bytes(ready_tag, encoding='utf8'))

            feedback = conn.recv(1024)
            feedback = str(feedback, encoding='utf8')

            if feedback.startswith('Start'):
                conn.send(send_data)

        except Exception as ex:
            break

    conn.close()
