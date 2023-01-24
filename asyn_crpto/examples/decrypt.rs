
use rsa::{pkcs8::DecodePrivateKey, Oaep};
use rsa::{RsaPrivateKey};
const RSA_2048_PRIV_DER: &[u8] = include_bytes!("../private_key.der");
fn main() {
   
    
    let rsa_private_key = RsaPrivateKey::from_pkcs8_der(RSA_2048_PRIV_DER).unwrap();
    // add vec which you get code from encrypt.rs here(only for rust)
    //let message:Vec<u8>=vec![40, 145, 1, 2, 102, 99, 12, 239, 15, 61, 183, 74, 245, 105, 33, 198, 224, 114, 108, 94, 249, 154, 159, 241, 120, 93, 33, 209, 25, 196, 33, 204, 67, 248, 231, 110, 79, 18, 102, 159, 124, 13, 41, 42, 234, 154, 8, 229, 208, 114, 171, 150, 212, 140, 221, 159, 10, 117, 106, 26, 229, 149, 236, 164, 39, 32, 243, 15, 73, 136, 56, 248, 79, 195, 150, 66, 85, 151, 142, 129, 240, 240, 168, 252, 242, 107, 228, 21, 229, 66, 202, 147, 3, 204, 23, 23, 166, 143, 249, 64, 14, 217, 162, 8, 234, 110, 221, 193, 0, 255, 53, 32, 148, 249, 166, 44, 113, 147, 244, 176, 207, 149, 103, 127, 92, 25, 165, 138, 195, 242, 242, 139, 119, 189, 188, 111, 33, 48, 119, 181, 151, 139, 204, 29, 25, 182, 204, 196, 90, 90, 130, 5, 252, 246, 144, 18, 109, 246, 205, 196, 238, 121, 201, 11, 226, 191, 248, 77, 22, 23, 104, 181, 75, 252, 222, 230, 175, 56, 90, 130, 17, 178, 214, 15, 86, 146, 37, 67, 150, 227, 188, 146, 7, 174, 247, 123, 0, 167, 247, 122, 27, 231, 149, 8, 239, 89, 93, 210, 136, 137, 91, 35, 152, 254, 80, 56, 76, 113, 92, 184, 139, 242, 96, 152, 135, 212, 100, 217, 21, 15, 126, 81, 26, 9, 238, 141, 92, 107, 37, 131, 36, 168, 84, 177, 228, 57, 89, 194, 187, 105, 45, 172, 242, 33, 201, 27]; // encrpted data 
    
    // add hex value which you recieved from js code(only for js)
    let message=hex::decode("05bfb9124f7a071aa0d2b84337db568b295fd2dfa78cd3b3bebae87eed38709e07adee24dbb7cd547a33ba673b8556049a0e7c6131498c3b9bca72038f5c2d263d5f0848d036c59d32f37455fcd7922975cb1d7ec0d30cd4a075ef2dff333e2389cdb5988a3c008e38d26baf96d13fee6e026baddb1253b7e569c91292e51fa0b9b1a4b07dbe0b27ffa63c10e2032db2efb99c29fa589381412c51a7eefeca208fd6f23ba4f0db759d3cc9bc3906507c628d87f7eb1f372508f4815b0917d2229a671ed3d7f6494fd2152b93d3c9d60f61429abe1f14d4d3adcaa70ec9ca9d311f6ebf8575ec9824548f1bd7b6a27cb465be0a4d23e6238fd2ec3d3231b98fb5").expect("hex error");
    println!("decoded data: {}", decode(message, rsa_private_key));
}

fn decode(data:Vec<u8>, rsa_private_key: RsaPrivateKey)->String{
    let padding = Oaep::new::<sha1::Sha1>();
    let dec_data = rsa_private_key
        .decrypt(padding, &data)
        .expect("failed to decrypt");




    let docode_str = String::from_utf8(dec_data).unwrap();

    return docode_str;

}