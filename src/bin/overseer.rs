extern crate overseer;

fn main() {
    // Execute the overseer driver
    overseer::Overseer::run(
    	overseer::Options::from_args(
    		std::os::args()));
}
