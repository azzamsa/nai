# Changelog

All notable changes to this project will be documented in this file.

## [2.0.0] - 2024-08-07

### Features

- Port chrono to jiff ([567a0b0](https://github.com/azzamsa/nai/commit/567a0b005183c1fdc000b8555a488aae87925170))
  - **BREAKING!** ⚠️ : port chrono to jiff

## [1.1.1] - 2023-07-22

### Bug fixes

- Can't show bright color ([2f29b1c](https://github.com/azzamsa/nai/commit/2f29b1ce7bd304789f5505003b729fd413a8d1c5))

## [1.1.0] - 2023-06-23

### Bug fixes

- Let user manage the newline ([b35a6dd](https://github.com/azzamsa/nai/commit/b35a6dd49c0637f60cede6cae32c23a7408029a0))

## [1.0.0] - 2023-06-21

### Features

- Accept anything in string ([1605d53](https://github.com/azzamsa/nai/commit/1605d53f8318d76548ff916198a32e438bd817a8))

## [0.9.0] - 2023-06-21

### Bug fixes

- Don't hard code style ([57529e9](https://github.com/azzamsa/nai/commit/57529e919ea193943833ce09cadc52e40879f36a))

## [0.8.0] - 2023-06-21

### Features

- Accept any character in `format` ([1c0f27a](https://github.com/azzamsa/nai/commit/1c0f27a13cd8488b6520c9c3a0662964b5320af9))

## [0.7.0] - 2023-06-20

### Features

- Support all `owo` effects ([4d0fc98](https://github.com/azzamsa/nai/commit/4d0fc983d367533d08ee0d143f26b18c9091902b))

## [0.6.0] - 2023-06-20

### Features

- Support all `owo-colors` ([0b6c766](https://github.com/azzamsa/nai/commit/0b6c766d87f2f19e96b149b7285ef76c276e1424))

## [0.5.0] - 2023-06-20

### Features

- Support color in variable ([d6a65ab](https://github.com/azzamsa/nai/commit/d6a65ab57b57afac59ece67c3d96a99160a5b303))
- Support color in literal string ([8c9c29a](https://github.com/azzamsa/nai/commit/8c9c29a1762783a40433513a7b7ff530e0232c09))
- Support literal string ([641e794](https://github.com/azzamsa/nai/commit/641e794ac42d2d947772f6ac4b7b3b6b2fff1561))

## [0.4.0] - 2023-06-20

### Features

- Support newline ([b86d9a7](https://github.com/azzamsa/nai/commit/b86d9a7f95ccb12111416571df104c0189990cbb))

### Bug fixes

- Migrate to `chrono` ([9b0ddd0](https://github.com/azzamsa/nai/commit/9b0ddd0321c9f7c59df65d9d0bd5dfaa3b35eb08))

  `time` has multi-threading issue when getting local offset.

## [0.3.0] - 2023-06-20

### Features

- Support punctuation ([22416ed](https://github.com/azzamsa/nai/commit/22416edbbec93e7ab0dd87810fb5b793efd490f2))

### Bug fixes

- Unreachable input ([38ba0e0](https://github.com/azzamsa/nai/commit/38ba0e0f9419a01dda624fdee7c9a671f3a91763))

## [0.2.1] - 2023-06-20

### Bug fixes

- Prettify `invalid syntax` error ([5df6c5a](https://github.com/azzamsa/nai/commit/5df6c5afc18703120c290318207420a23c6fa7ff))
- Don't stop at invalid syntax ([ae38905](https://github.com/azzamsa/nai/commit/ae38905b333b970abb5a099db924506c5ffc64c0))

  Read the whole input even if it contains error.

## [0.2.0] - 2023-06-20

### Bug fixes

- Accept custom config location ([1c5c241](https://github.com/azzamsa/nai/commit/1c5c241af99692a057f12911dd32a264738500b7))
