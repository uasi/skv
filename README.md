# skv - Something from Key-Value

skv converts human-writable key-value pairs into JSON and some other stuff.

## Synopsis

```
$ skv key=value 'what$#\=!=?@"' 'raw_json:=[42]' | jq .
{
  "key": "value",
  "raw_json": [
    42
  ],
  "what$#=!": "?@\""
}

$ skv --curl User-Agent:skv key=value 'what$#\=!=?@"'
-d key=value&what%24%23%3D%21=%5C%5C%3F%40%22 -H 'User-Agent: skv'

$ curl $(skv --curl User-Agent:skv key=value) -X POST http://example.com/
...
```

## License

skv is released under the MIT license.

## Contributing

1. Fork it
2. Create your feature branch (`git checkout -b my-new-feature`)
3. Commit your changes (`git commit -am 'Add some feature'`)
4. Push to the branch (`git push origin my-new-feature`)
5. Create new Pull Request
