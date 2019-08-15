# futures_cbor_codec

[![standard-readme compliant](https://img.shields.io/badge/readme%20style-standard-brightgreen.svg?style=flat-square)](https://github.com/RichardLitt/standard-readme)
[![Build Status](https://api.travis-ci.org/najamelan/futures_cbor_codec.svg?branch=master)](https://travis-ci.org/najamelan/futures_cbor_codec)
[![Docs](https://docs.rs/futures_cbor_codec/badge.svg)](https://docs.rs/futures_cbor_codec)
[![crates.io](https://img.shields.io/crates/v/futures_cbor_codec.svg)](https://crates.io/crates/futures_cbor_codec)


> A codec for framing AsyncRead/AsyncWrite from the futures lib with serde-cbor

This rust crate integrates the [`serde-cbor`](https://crates.io/crates/serde-cbor) into a codec
(`Decoder` and `Encoder`) of [`future-codec`](https://crates.io/crates/futures-codec). This allows
turning an AsyncRead/AsyncWrite into a stream and sink of rust objects that implement Serialize/Deserialize from serde.

This is a fork from [tokio-serde-cbor](https://crates.io/crates/tokio-serde-cbor).
It turned out to work unchanged for futures-codec. All the credit for this functionality should go to @vorner.

## Table of Contents

- [Install](#install)
  - [Upgrade](#upgrade)
  - [Dependencies](#dependencies)
- [Usage](#usage)
  - [Basic Example](#basic-example)
  - [API](#api)
- [Contributing](#contributing)
  - [Code of Conduct](#code-of-conduct)
- [License](#license)


## Install
With [cargo add](https://github.com/killercup/cargo-edit):
`cargo add futures_cbor_codec`

With [cargo yaml](https://gitlab.com/storedbox/cargo-yaml):
```yaml
dependencies:

  futures_cbor_codec: ^0.1
```

With raw Cargo.toml
```toml
[dependencies]

   futures_cbor_codec = "^0.1"
```

### Upgrade

Please check out the [changelog](https://github.com/najamelan/futures_cbor_codec/blob/master/CHANGELOG.md) when upgrading.

### Dependencies

This crate has few dependiencies. Cargo will automatically handle it's dependencies for you.

There are no optional features.


## Usage

This crate provides a codec for framing information as CBOR encoded messages. It allows
encoding and decoding arbitrary [serde](https://serde.rs) ready types. It can be used by
plugging the codec into the connection's `framed` method to get stream and sink of the desired
items.

The encoded and decoded items are independent (you may want to encode references and decode
owned data, or the protocol might be asymetric). If you want just one direction, you can use
[`Decoder`](struct.Decoder.html) or [`Encoder`](struct.Encoder.html). If you want both, you
better use [`Codec`](struct.Codec.html).

Note that this is useful if the CBOR itself defines the frames. If the messages are delimited
in some other way (eg. length-prefix encoding) and CBOR is only the payload, you'd use a codec
for the other framing and use `.map` on the received stream and sink to convert the messages.

Please have a look in the [examples directory of the repository](https://github.com/najamelan/futures_cbor_codec/tree/master/examples).

This crate works on WASM.


### Basic example
```rust
//! Example demonstration how to use the codec with futures 0.3 networking and the futures_codec crate.
//! Run with `cargo run --example basic`.
//
#![feature(async_await)]


use
{
  async_ringbuffer   :: { *                                      } ,
  futures            :: { SinkExt, StreamExt, executor::block_on } ,
  futures_codec      :: { FramedRead, FramedWrite                } ,
  futures_cbor_codec :: { Codec                                  } ,
  std                :: { collections::HashMap                   } ,
};


// We create some test data to serialize. This works because Serde implements
// Serialize and Deserialize for HashMap, so the codec can frame this type.
//
type TestData = HashMap<String, usize>;


/// Something to test with. It doesn't really matter what it is.
//
fn test_data() -> TestData
{
  let mut data = HashMap::new();

  data.insert( "hello".to_string(), 42 );
  data.insert( "world".to_string(), 0  );

  data
}



// In a real life scenario the sending and receiving end usually are in different processes.
// We could simulate that somewhat by putting them in separate async blocks and spawning those,
// but since we only send in one direction, I chose to keep it simple.
//
// Obviously in production code you should do some real error handling rather than using
// `expect`. However for this example, almost any error would fatal, so we might as well.
//
fn main()
{
  let program = async
  {
    let (write, read) = ring_buffer(32);

    // Type annotations are needed unfortunately. The compiler won't infer them just yet.
    // On an object that implements both `AsyncRead` + `AsyncWrite`, we could use the
    // `Framed` struct from futures_codec, but since ringbuffer doesn't have a unified
    // object, we construct `FramedRead` and `FramedWrite` separately.
    //
    let mut reader = FramedRead ::new( read , Codec::<TestData, TestData>::new() );
    let mut writer = FramedWrite::new( write, Codec::<TestData, TestData>::new() );

    writer.send( test_data() ).await.expect( "send message1" );
    writer.send( test_data() ).await.expect( "send message2" );
    writer.close().await.expect( "close sender" );


    while let Some(msg) = reader.next().await.transpose().expect( "receive message" )
    {
      println!( "Received: {:#?}", msg );
    }
  };

  block_on( program );
}
```

## API

Api documentation can be found on [docs.rs](https://docs.rs/futures_cbor_codec).


## Contributing

This repository accepts contributions. Ideas, questions, feature requests and bug reports can be filed through github issues.

Pull Requests are welcome on github. By commiting pull requests, you accept that your code might be modified and reformatted to fit the project coding style or to improve the implementation. Please discuss what you want to see modified before filing a pull request if you don't want to be doing work that might be rejected.

Please file PR's against the `dev` branch, don't forget to update the changelog and the documentation.

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms
or conditions.

### Testing

`cargo test`

On wasm, after [installing wasm-pack](https://rustwasm.github.io/wasm-pack/):

`wasm-pack test --firefox --headless`

or

`wasm-pack test --chrome --headless`


### Code of conduct

Any of the behaviors described in [point 4 "Unacceptable Behavior" of the Citizens Code of Conduct](http://citizencodeofconduct.org/#unacceptable-behavior) are not welcome here and might get you banned. If anyone including maintainers and moderators of the project fail to respect these/your limits, you are entitled to call them out.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
