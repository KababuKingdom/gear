use gtest::{Log, Program, System};
use hello_world::{ActorId, InputMessages};

#[test]
fn hello_test() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);

    let res = program.send_bytes(2, String::from("Hello"));
    assert!(!res.main_failed());
    assert!(res.log().is_empty());

    // test `SendHelloTo`
    let hello_recipient: ActorId = 4.into();
    let res = program.send(2, InputMessages::SendHelloTo(hello_recipient));
    let expected_log = Log::builder().dest(hello_recipient).payload(String::from("Hello"));
    assert!(res.contains(&expected_log))
}
