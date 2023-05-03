#![no_std]
use gstd::{msg, prelude::*, debug};
use sp_io::misc::print_utf8;
use parity_scale_codec::{Decode, Encode}!;
type ActorId = u64;

#[derive(Encode, Decode, TypeInfo)]
enum InputMessages {
    SendHelloTo(ActorId),
    SendHelloReply,
}

static mut GREETING: Option<String> = None;

#[no_mangle]
extern "C" fn init() {
    let init_message = msg::load().expect("Failed to load init message");
    let greeting = match init_message.try_into() {
        Ok(s) => s,
        Err(_) => panic!("Init message is not a valid string"),
    };
    debug!("Program was initialized with message {:?}", greeting);
    unsafe { GREETING = Some(greeting) };
}


#[no_mangle]
extern "C" fn handle() {
    let input_message: InputMessages =
        msg::load().expect("Error in loading InputMessages");
    let greeting = unsafe { GREETING.as_mut().expect("The contract is not initialized") };
    match input_message {
        InputMessages::SendHelloTo(account) => {
            debug!("Message: SendHelloTo {:?}", account);
            msg::send(account, greeting(), 0).expect("Error in sending Hello message to account");
        }
        InputMessages::SendHelloReply => {
            debug!("Message: SendHelloReply");
            msg::reply(greeting(), 0).expect("Error in sending reply");
        }
    }
}

#[no_mangle]
extern "C" fn test_send_hello_to() {
    let hello_recipient: ActorId = 4;
    let input_message = InputMessages::SendHelloTo(hello_recipient);
    let result = msg::send(input_message);
    debug!("Result: {:?}", result);
}

#[no_mangle]
extern "C" fn test_send_hello_reply() {
    let input_message = InputMessages::SendHelloReply;
    let result = msg::reply(input_message);
    debug!("Result: {:?}", result);
}

#[no_mangle]
extern "C" fn test_print_utf8() {
    let s = "hello world";
    print_utf8(s.as_bytes());
}
