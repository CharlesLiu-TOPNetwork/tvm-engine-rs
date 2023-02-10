# Precompiled Contract

## Overview

| Address | Name       | Minimum Gas | Input                               | Ouput           | Description                                                                     |
| ------- | ---------- | ----------- | ----------------------------------- | --------------- | ------------------------------------------------------------------------------- |
| 0x01    | ecRecover  | 3000        | `hash` `v` `r` `s`                  | `publicAddress` | Elliptic curve digital signature algorithm (ECDSA) public key recovery function |
| 0x02    | SHA2-256   | 60+         | `data`                              | `hash`          | Hash function                                                                   |
| 0x03    | RIPEMD-160 | 600+        | `data`                              | `hash`          | Hash function                                                                   |
| 0x04    | identity   | 15+         | `data`                              | `data`          | Returns the input                                                               |
| 0x05    | modexp     | 200+        | `Bsize` `Esize` `Msize` `B` `E` `M` | `value`         | Arbitrary-precision exponentiation under modulo                                 |
| 0x06    | ecAdd      | 150+        | `x1` `y1` `x2` `y2`                 | `x` `y`         | Point addition (ADD) on the elliptic curve 'alt_bn128'                          |
| 0x07    | ecMul      | 6000+       | `x1` `y1` `s`                       | `x` `y`         | Scalar multiplication (MUL) on the elliptic curve 'alt_bn128'                   |
| 0x08    | ecPairing  | 45000+      | `x1` `y1` `x2` `y2` `...` `xk` `yk` | `success`       | Bilinear function on groups on the elliptic curve 'alt_bn128'                   |
| 0x09    | blake2f    | 0+          | `rounds` `h` `m` `t` `f`            | `h`             | Compression function F used in the BLAKE2 cryptographic hashing algorithm       |


## Details

### 0x01. ecRecover

information about ECDSA at [here](https://en.wikipedia.org/wiki/Elliptic_Curve_Digital_Signature_Algorithm)

#### Input

| Byte range           | Name   | Description                                          |
| -------------------- | ------ | ---------------------------------------------------- |
| [0; 31] (32 bytes)   | `hash` | Keccack-256 hash of the transaction                  |
| [32; 63] (32 bytes)  | `v`    | Recovery identifier, expected to be either 27 or 28  |
| [64; 95] (32 bytes)  | `r`    | x-value, expected to be in the range ]0; secp256k1n[ |
| [96; 127] (32 bytes) | `s`    | Expected to be in the range ]0; secp256k1n[          |

#### Output

| Byte range         | Name          | Description                                             |
| ------------------ | ------------- | ------------------------------------------------------- |
| [0; 31] (32 bytes) | publicAddress | The recovered 20-byte address right aligned to 32 bytes |

If an address cannot be recovered, or not enough gas was given, then there is no return data. 

Please note, that the return is the address that issued the signature but it won't verify the signature.

#### Example
| Input                                                              | Output                                     |
| ------------------------------------------------------------------ | ------------------------------------------ |
| 0x456e9aea5e197a1f1af7a3e85a3212fa4049a3ba34c2289b4c860fc0b0c64ef3 | 0x7156526fbd7a3c72969b54f64e42c10fbb768c8a |
| 28                                                                 |
| 0x9242685bf161793cc25603c231bc2f568eb630ea16aa137d2664ac8038825608 |
| 0x4f8ae3bd7535248d0bd448298cc2e2071e56992d0774dc340c368ae950852ada |

### 0x02. SHA2

#### Inputs

| Byte range  | Name | Description                |
| ----------- | ---- | -------------------------- |
| [0; length[ | data | Data to hash with SHA2-256 |

#### Output

| Byte range         | Name | Description     |
| ------------------ | ---- | --------------- |
| [0; 31] (32 bytes) | hash | The result hash |

If not enough gas was given, then there is no return data.

#### Example

| Input | Output                                                           |
| ----- | ---------------------------------------------------------------- |
| 0xFF  | a8100ae6aa1940d0b663bb31cd466142ebbdbd5187131b92d93818987832eb89 |

#### Gas

``` TEXT
data_word_size = (data_size + 31) / 32

static_gas = 60
dynamic_gas = 12 * data_word_size
```


### 0x03. RIPEMD


### 0x04. identity


### 0x05. modexp


### 0x06. ecAdd


### 0x07. ecMul


### 0x08. ecPairing


### 0x09. blake2f

