# Precompiled Contract

## Overview

| Address | Name       | Minimum Gas | Input                               | Ouput           | Description                                                                     |
| ------- | ---------- | ----------- | ----------------------------------- | --------------- | ------------------------------------------------------------------------------- |
| 0x01    | ecRecover  | 3000        | `hash` `v` `r` `s`                  | `publicAddress` | Elliptic curve digital signature algorithm (ECDSA) public key recovery function |
| 0x02    | SHA2-256   | 60+         | `data`                              | `hash`          | Hash function                                                                   |
| 0x03    | RIPEMD-160 | 600+        | `data`                              | `hash`          | Hash function                                                                   |
| 0x04    | identity   | 15+         | `data`                              | `data`          | Returns the input                                                               |
| 0x05    | modexp     | 200+        | `Bsize` `Esize` `Msize` `B` `E` `M` | `value`         | Arbitrary-precision exponentiation under modulo                                 |
| 0x06    | ecAdd      | 150?        | `x1` `y1` `x2` `y2`                 | `x` `y`         | Point addition (ADD) on the elliptic curve 'alt_bn128'                          |
| 0x07    | ecMul      | 6000?       | `x1` `y1` `s`                       | `x` `y`         | Scalar multiplication (MUL) on the elliptic curve 'alt_bn128'                   |
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
| [0; length] | data | Data to hash with SHA2-256 |

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

#### Inputs

| Byte range  | Name | Description                  |
| ----------- | ---- | ---------------------------- |
| [0; length] | data | Data to hash with RIPEMD-160 |

#### Output

| Byte range         | Name | Description                                       |
| ------------------ | ---- | ------------------------------------------------- |
| [0; 31] (32 bytes) | hash | The result 20-byte hash right aligned to 32 bytes |

If not enough gas was given, then there is no return data.

#### Example

| Input | Output                                   |
| ----- | ---------------------------------------- |
| 0xFF  | 2c0c45d3ecab80fe060e5f1d7057cd2f8de5e557 |

#### Gas

``` TEXT
data_word_size = (data_size + 31) / 32

static_gas = 600
dynamic_gas = 120 * data_word_size
```


### 0x04. identity

The identity function is typically used to copy a chunk of memory.

#### Inputs

| Byte range  | Name | Description    |
| ----------- | ---- | -------------- |
| [0; length] | data | Data to return |

#### Output

| Byte range  | Name | Description     |
| ----------- | ---- | --------------- |
| [0; length] | data | Data from input |

If not enough gas was given, then there is no return data.

#### Example

| Input | Output |
| ----- | ------ |
| 0xFF  | 0xFF   |

#### Gas

``` TEXT
data_word_size = (data_size + 31) / 32

static_gas = 15
dynamic_gas = 3 * data_word_size
```


### 0x05. modexp

#### Inputs

| Byte range                                       | Name  | Description                                                    |
| ------------------------------------------------ | ----- | -------------------------------------------------------------- |
| [0; 31] (32 bytes)                               | Bsize | Byte size of B                                                 |
| [32; 63] (32 bytes)                              | Esize | Byte size of E                                                 |
| [64; 95] (32 bytes)                              | Msize | Byte size of M                                                 |
| [96; 96 + Bsize]                                 | B     | Base as unsigned integer                                       |
| [96 + Bsize; 96 + Bsize + Esize]                 | E     | Exponent as unsigned integer, if zero, then B ** E will be one |
| [96 + Bsize + Esize; 96 + Bsize + Esize + Msize] | M     | Modulo as unsigned integer, if zero, then returns zero         |

#### Output

| Byte range | Name  | Description                                                   |
| ---------- | ----- | ------------------------------------------------------------- |
| [0; mSize] | value | Result of the computation, with the same number of bytes as M |

If not enough gas was given, then there is no return data.

#### Example

| Input | Output |
| ----- | ------ |
| 1     | 8      |
| 1     |
| 1     |
| 8     |
| 9     |
| 10    |

#### Gas

``` TEXT
max_length = max(Bsize, Msize)
words = (max_length + 7) / 8
multiplication_complexity = words**2

iteration_count = 0
if Esize <= 32 and exponent == 0: iteration_count = 0
elif Esize <= 32: iteration_count = exponent.bit_length() - 1
elif Esize > 32: iteration_count = (8 * (Esize - 32)) + ((exponent & (2**256 - 1)).bit_length() - 1)
calculate_iteration_count = max(iteration_count, 1)

static_gas = 0
dynamic_gas = max(200, multiplication_complexity * iteration_count / 3)
```


### 0x06. ecAdd

The point at infinity is encoded with both field x and y at 0.

#### Inputs

| Byte range           | Name | Description                                                        |
| -------------------- | ---- | ------------------------------------------------------------------ |
| [0; 31] (32 bytes)   | x1   | X coordinate of the first point on the elliptic curve 'alt_bn128'  |
| [32; 63] (32 bytes)  | y1   | Y coordinate of the first point on the elliptic curve 'alt_bn128'  |
| [64; 95] (32 bytes)  | x2   | X coordinate of the second point on the elliptic curve 'alt_bn128' |
| [96; 128] (32 bytes) | y2   | Y coordinate of the second point on the elliptic curve 'alt_bn128' |

#### Output

| Byte range          | Name | Description                                                        |
| ------------------- | ---- | ------------------------------------------------------------------ |
| [0; 31] (32 bytes)  | x    | X coordinate of the result point on the elliptic curve 'alt_bn128' |
| [32; 63] (32 bytes) | y    | Y coordinate of the result point on the elliptic curve 'alt_bn128' |
If the input is not valid, or if not enough gas was given, then there is no return data.

#### Example

| Input | Output                                                             |
| ----- | ------------------------------------------------------------------ |
| 1     | 0x030644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd3 |
| 2     | 0x15ed738c0e0a7c92e7845f96b2ae9c0a68a6a449e3538fc7ff3ebf7a5a18a2c4 |
| 1     |
| 2     |

#### Gas

The gas cost is fixed at 150. However, if the input does not allow to compute a valid result, all the gas sent is consummed.


### 0x07. ecMul

The point at infinity is encoded with both field x and y at 0.

#### Inputs

| Byte range          | Name | Description                                                       |
| ------------------- | ---- | ----------------------------------------------------------------- |
| [0; 31] (32 bytes)  | x1   | X coordinate of the first point on the elliptic curve 'alt_bn128' |
| [32; 63] (32 bytes) | y1   | Y coordinate of the first point on the elliptic curve 'alt_bn128' |
| [64; 95] (32 bytes) | s    | Scalar to use for the multiplication                              |

#### Output

| Byte range          | Name | Description                                                        |
| ------------------- | ---- | ------------------------------------------------------------------ |
| [0; 31] (32 bytes)  | x    | X coordinate of the result point on the elliptic curve 'alt_bn128' |
| [32; 63] (32 bytes) | y    | Y coordinate of the result point on the elliptic curve 'alt_bn128' |

If the input is not valid, or if not enough gas was given, then there is no return data.

#### Example

| Input | Output                                                             |
| ----- | ------------------------------------------------------------------ |
| 1     | 0x030644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd3 |
| 2     | 0x15ed738c0e0a7c92e7845f96b2ae9c0a68a6a449e3538fc7ff3ebf7a5a18a2c4 |
| 2     |

#### Gas

The gas cost is fixed at 6000. However, if the input does not allow to compute a valid result, all the gas sent is consummed.


### 0x08. ecPairing

The point at infinity is encoded with both field x and y at 0.

#### Inputs
The input must always be a multiple of 6 * 32-byte values. 0 inputs is valid and returns 1. One set of inputs is defined as follows:

| Byte range            | Name |
| --------------------- | ---- |
| [0; 31] (32 bytes)    | x1   |
| [32; 63] (32 bytes)   | y1   |
| [64; 95] (32 bytes)   | x3   |
| [96; 127] (32 bytes)  | x2   |
| [128; 159] (32 bytes) | y3   |
| [160; 191] (32 bytes) | y2   |

#### Output

| Byte range         | Name    | Description                                 |
| ------------------ | ------- | ------------------------------------------- |
| [0; 31] (32 bytes) | success | 1 if the pairing was a success, 0 otherwise |

If the input is not valid, or if not enough gas was given, then there is no return data.

#### Example

| Input                                                              | Output |
| ------------------------------------------------------------------ | ------ |
| 0x2cf44499d5d27bb186308b7af7af02ac5bc9eeb6a3d147c186b21fb1b76e18da | 1      |
| 0x2c0f001f52110ccfe69108924926e45f0b0c868df0e7bde1fe16d3242dc715f6 |
| 0x1fb19bb476f6b9e44e2a32234da8212f61cd63919354bc06aef31e3cfaff3ebc |
| 0x22606845ff186793914e03e21df544c34ffe2f2f3504de8a79d9159eca2d98d9 |
| 0x2bd368e28381e8eccb5fa81fc26cf3f048eea9abfdd85d7ed3ab3698d63e4f90 |
| 0x2fe02e47887507adf0ff1743cbac6ba291e66f59be6bd763950bb16041a0a85e |
| 0x0000000000000000000000000000000000000000000000000000000000000001 |
| 0x30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd45 |
| 0x1971ff0471b09fa93caaf13cbf443c1aede09cc4328f5a62aad45f40ec133eb4 |
| 0x091058a3141822985733cbdddfed0fd8d6c104e9e9eff40bf5abfef9ab163bc7 |
| 0x2a23af9a5ce2ba2796c1f4e453a370eb0af8c212d9dc9acd8fc02c2e907baea2 |
| 0x23a8eb0b0996252cb548a4487da97b02422ebc0e834613f954de6c7e0afdc1fc |

#### Gas

``` TEXT
34000/pair + 45000
```


### 0x09. blake2f

#### Inputs

| Byte range            | Name   | Description                                                     |
| --------------------- | ------ | --------------------------------------------------------------- |
| [0; 3] (4 bytes)      | rounds | Number of rounds (big-endian unsigned integer)                  |
| [4; 67] (64 bytes)    | h      | State vector (8 8-byte little-endian unsigned integer)          |
| [68; 195] (128 bytes) | m      | Message block vector (16 8-byte little-endian unsigned integer) |
| [196; 211] (16 bytes) | t      | Offset counters (2 8-byte little-endian integer)                |
| [212; 212] (1 bytes)  | f      | Final block indicator flag (0 or 1)                             |

#### Output

| Byte range         | Name | Description                                            |
| ------------------ | ---- | ------------------------------------------------------ |
| [0; 63] (64 bytes) | h    | State vector (8 8-byte little-endian unsigned integer) |

If the input is not valid, or if not enough gas was given, then there is no return data.

#### Gas

``` TEXT
static_gas = 0
dynamic_gas = rounds * 1
However, if the input does not allow to compute a valid result, all the gas sent is consummed.
```