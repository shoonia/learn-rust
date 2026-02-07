echo "\n"
echo "Using GET request with query parameters:"

curl "http://localhost:3000/calculate?a=10&b=20&operation=add"

echo "\n"
echo "Using POST request with JSON body:"

curl -s "http://localhost:3000/calculate" -X POST -H "Content-Type: application/json" -d '{"a": 10, "b": 20, "operation": "add"}'
