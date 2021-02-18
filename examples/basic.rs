//! Demonstration how to use the codec with futures 0.3 networking and the futures_codec crate.
//! Run with `cargo run --example basic`.
//
use
{
	futures_ringbuf    :: { *                                      } ,
	futures            :: { SinkExt, StreamExt, executor::block_on } ,
	futures_cbor_codec :: { Codec                                  } ,
	std                :: { collections::HashMap                   } ,

	// crate asynchronous-codec, but in my Cargo.toml I shorten the name...
	//
	async_codec        :: { Framed                                 } ,
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
		// This creates a pair of TCP domain sockets that are connected together.
		//
		let (sender_socket, receiver_socket) = Endpoint::pair( 128, 128 );

		// Type annotations are needed unfortunately. The compiler won't infer them just yet.
		//
		let mut reader = Framed::new( receiver_socket, Codec::<TestData, TestData>::new() );
		let mut writer = Framed::new( sender_socket  , Codec::<TestData, TestData>::new() );

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


