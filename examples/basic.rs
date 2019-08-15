//! Example demonstration how to use the codec with futures 0.3 networking and the futures_codec crate.
//! Run with `cargo run --example basic`.
//
#![feature(async_await)]


use
{
	futures_ringbuf    :: { *                                                    } ,
	futures            :: { SinkExt, StreamExt, AsyncReadExt, executor::block_on } ,
	futures_codec      :: { FramedRead, FramedWrite                              } ,
	futures_cbor_codec :: { Decoder, Encoder                                     } ,
	std                :: { collections::HashMap                                 } ,
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
		let (read, write) = RingBuffer::new(32).split();

		// Type annotations are needed unfortunately. The compiler won't infer them just yet.
		// On an object that implements both `AsyncRead` + `AsyncWrite`, we could use the
		// `Framed` struct from futures_codec and the Codec struct from futures_cbor_codec,
		// but since ringbuffer doesn't have a unified object, we construct `FramedRead` and
		// `FramedWrite` separately.
		//
		let mut reader = FramedRead ::new( read , Decoder::<TestData>::new() );
		let mut writer = FramedWrite::new( write, Encoder::<TestData>::new() );

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


