//! Example demonstration how to use the codec with futures 0.3 networking and the futures_codec crate.
//! Run with `cargo run --example basic`.
//
#![feature(async_await)]





// In a real life scenario the sending and receiving end usually are in different processes.
// We could simulate that somewhat by putting them in separate async blocks and spawning those,
// but since we only send in one direction, I chose to keep it simple.
//
// Obviously in production code you should do some real error handling rather than using
// `expect`. However for this example, almost any error would fatal, so we might as well.
//
#[ cfg(not( target_arch = "wasm32" )) ]
//
fn main()
{
	use
	{
		futures            :: { SinkExt, StreamExt, TryStreamExt, executor::block_on } ,
		futures_codec      :: { Framed                                               } ,
		futures_cbor_codec :: { Codec                                                } ,
		std                :: { collections::HashMap                                 } ,
		romio              :: { TcpListener, TcpStream                               } ,
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



	/// Creates a connected pair of tcp sockets.
	//
	async fn socket_pair() -> Result<(TcpStream, TcpStream), Box<dyn std::error::Error + Send + Sync> >
	{
		// port 0 = let the OS choose
		//
		let mut listener = TcpListener::bind   ( &"127.0.0.1:0".parse()? )? ;
		let     stream1  = TcpStream  ::connect( &listener.local_addr()? )  ;

		let mut incoming = listener.incoming();
		let     stream2  = incoming.next();

		Ok(( stream1.await?, stream2.await.expect( "some connection")? ))
	}


	let program = async
	{
		// This creates a pair of TCP domain sockets that are connected together.
		//
		let (sender_socket, receiver_socket) = socket_pair().await.expect( "create socketpair" );

		// Type annotations are needed unfortunately. The compiler won't infer them just yet.
		//
		let mut receiver = Framed::new( receiver_socket, Codec::<TestData, TestData>::new() );
		let mut sender   = Framed::new( sender_socket  , Codec::<TestData, TestData>::new() );

		sender.send( test_data() ).await.expect( "send message1" );
		sender.send( test_data() ).await.expect( "send message2" );
		sender.close().await.expect( "close sender" );

		// Needed because otherwise romio doesn't close the tcpstream:
		// https://github.com/withoutboats/romio/issues/81
		//
		drop( sender );


		while let Some(msg) = receiver.try_next().await.expect( "read from stream" )
		{
			println!( "Received: {:#?}", msg );
		}
	};

	block_on( program );
}


#[ cfg( target_arch = "wasm32" )]
fn main()
{
	println!( "This example cannot be run on wasm32-* targets" );
}
