Demo project for reproducing brotli compression issue in rust actix-web framework.

Response is truncated when using brotli for encoding relatively big files (~140kb in this case).

---

1. run server (`cargo run`)
2. a. open url in browser (`locahost:88`)

    open dev tools and make sure browser requests `big.js` file with `Accept-Encoding: br` header.  
    - Chrome v79 sends `Accept-Encoding: gzip, deflate, br`
    - Firefox v70 sends `Accept-Encoding: gzip, deflate` (it supports brotli, but not for http localhost, so it wont reproduce here)  

    **Result**: browser receives js file but it is truncated, so there's js parsing error in console: `Uncaught SyntaxError: Unexpected end of input`.

    b. alternatively use CLI with curl and brotli decompression tool
   
    ```
    root@ubuntu:/# brotli -V
    brotli 1.0.4
    root@ubuntu:/# curl http://localhost:88/big.js -H "Accept-Encoding: br" | brotli --decompress
    ```

    **Result**: response file is truncated.

Response is not truncated when
- using no encoding (eg `curl http://localhost:88/big.js`)
- using gzip or deflate (eg. `curl http://localhost:88/big.js -H "Accept-Encoding: gzip" --compressed`)
- using brotli for small file (eg. `curl http://localhost:88/small.js -H "Accept-Encoding: br" | brotli --decompress`)

File sizes (uncompressed)
- small.js = 4.076 b
- big.js = 141.724 b