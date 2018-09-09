extern crate fbz;

fn main() {
    let events_loop = fbz::EventsLoop::new();
    for (num, monitor) in events_loop.get_available_monitors().enumerate() {
        println!("Monitor #{}: {:?}", num, monitor.get_name());
    }
}
