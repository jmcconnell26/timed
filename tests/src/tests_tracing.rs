use std::{thread, time};
use timed::timed;

#[allow(dead_code)]
fn sleep() {
    thread::sleep(time::Duration::from_millis(10));
}

#[timed(tracing=true)]
fn foo() {
    bar();
    sleep();
    baz();
}

#[timed(tracing=true)]
fn bar() {
    sleep();
    baz();
    sleep();
}

#[timed(tracing=true)]
fn baz() {
    sleep();
    foo::bar::baz::foobar();
}

pub mod foo {
    pub mod bar {
        pub mod baz {
            use timed::timed;
            #[timed(tracing=true)]
            pub fn foobar() {
                println!("Foobar");
            }
        }
    }
}

#[test]
#[timed(tracing=true)]
fn test_tracing() {

    // TODO: crate timed::tracing::init!("Test");
    let _trace = timed_tracing::Trace::new("Test".to_string());

    println!("Running main");
    sleep();
    foo();
}
