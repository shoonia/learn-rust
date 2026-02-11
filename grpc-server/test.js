await fetch('http://127.0.0.1:50051/com.servers.Greeter/SayHello', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/grpc-web+proto',
    'x-grpc-web': '1'
  },
  body: new Uint8Array([
    0, 0, 0, 0, 7,  // Header: No compression, length=7
    10, 5,          // Proto: Tag=10 (0x0A), String Len=5
    87, 111, 114, 108, 100 // Proto: "World" (UTF-8)
  ]),
}).then(res => res.text())
