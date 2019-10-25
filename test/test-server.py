import http.server
import json
import pprint
import sys

bind = sys.argv[1]
port = int(sys.argv[2])

handlers = {
    "ping": lambda _params: "pong",
    "add": lambda params: sum(params),
    "concat": lambda params: "".join(params),
    "echo": lambda params: params,
}

class TestServer(http.server.BaseHTTPRequestHandler):
    def do_POST(self):
        try:
            length = int(self.headers.get("Content-Length"))
            content = self.rfile.read(length)
            self.log_message("<-- %s", content)
            request = json.loads(content)
            method = request["method"]
            handler = handlers.get(method, None)
            if handler:
                result = handler(request["params"])
                response = {
                    "jsonrpc": "2.0",
                    "id": request["id"],
                    "result": result,
                }
            else:
                response = {
                    "jsonrpc": "2.0",
                    "id": request["id"],
                    "error": { "code": -32601, "message": "Method not found" }
                }
        except:
            response = {
                "jsonrpc": "2.0",
                "id": request["id"],
                "error": { "code": -1, "message": "Error" }
            }
        finally:
            dump = json.dumps(response)
            buffer = bytes(dump, encoding="utf8")
            self.send_response(200)
            self.send_header("Content-Type", "application/json")
            self.send_header("Content-Length", len(buffer))
            self.end_headers()
            self.log_message("--> %s", dump)
            self.wfile.write(buffer)

with http.server.HTTPServer((bind, port), TestServer) as httpd:
    httpd.serve_forever()