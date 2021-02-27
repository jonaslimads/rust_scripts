#[cfg(feature = "amqp")]
mod amqp;
#[cfg(feature = "amqp")]
use amqp::version;

#[cfg(feature = "kafka")]
mod kafka;
#[cfg(feature = "kafka")]
use kafka::version;

fn main() {
    println!("Hello, world! {}", version());
}
