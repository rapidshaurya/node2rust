# Asym Crpto
## This project will help you to encrypt and decrypt data using asymmetic crptography

## Run Code
- Run this code inside asyn_crpto
- For encrypt data
```
cargo run --example encrypt
```
- For decrypt data
```
cargo run --example decrypt
```

## encrypt data from javascript and then decrypt in rust
- Run this code inside node2rust
```
node index.js
```
- After running it copy the hex code and paste this code in asyn_crpto(examples->decrypt)
- Example
```
let message=hex::decode("05bfb9124f7a071aa0d2b84337db568b295fd2dfa78cd3b3bebae87eed38709e07adee24dbb7cd547a33ba673b8556049a0e7c6131498c3b9bca72038f5c2d263d5f0848d036c59d32f37455fcd7922975cb1d7ec0d30cd4a075ef2dff333e2389cdb5988a3c008e38d26baf96d13fee6e026baddb1253b7e569c91292e51fa0b9b1a4b07dbe0b27ffa63c10e2032db2efb99c29fa589381412c51a7eefeca208fd6f23ba4f0db759d3cc9bc3906507c628d87f7eb1f372508f4815b0917d2229a671ed3d7f6494fd2152b93d3c9d60f61429abe1f14d4d3adcaa70ec9ca9d311f6ebf8575ec9824548f1bd7b6a27cb465be0a4d23e6238fd2ec3d3231b98fb5").expect("hex error");
```
- Note the hex words should be in same line
## How to create openssl file for this project
- Private key generate
```
openssl genrsa -out key.pem 2048
openssl pkcs8 -topk8 -inform PEM -outform PEM -nocrypt -in key.pem -out private.pem
openssl asn1parse -in private.pem -out private_key.der
```
- pub key generate
```
openssl pkey -in private.pem -pubout -out pubkey.pem
openssl asn1parse -in pubkey.pem -out pub_key.der
```
#### Note
- Please add pub_key.der and private_key.der outside the src folder
- ⚠️ Please don't change cargo.toml dependencies
- Use RSA-2048 PKCS#8 private key encoded as ASN.1 DER
- Use RSA-2048 `SubjectPublicKeyInfo` encoded as ASN.1 DER
#### References 
- [RSA crate](https://crates.io/crates/rsa)
- [base64 crate](https://crates.io/crates/base64)
- [Openssl commands](https://www.openssl.org/docs/man3.0/man1/)

