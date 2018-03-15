# Single line excution.
find ./proto/greeter -name "*.proto" | xargs protoc -I "./proto/." -o "./greeter.desc"
