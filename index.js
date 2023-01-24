const NodeRsa = require('node-rsa');
var fs= require('fs')
const public_key =fs.readFileSync('pubkey.pem');
let rsakey = NodeRsa(public_key, {"encryptionScheme":"pkcs1_oaep", "signingScheme":"pss-sha1"});
const pk =rsakey.exportKey('pkcs8-public-der');
let message="write message here"
let encrpted_data=rsakey.encrypt(message, 'hex');
console.log(encrpted_data);

