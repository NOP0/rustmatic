use rustmatic_core::{ChannelContainer};
use rustmatic_runtime;

fn main() {
    
    let mut my_bool_container = ChannelContainer::<bool, bool>::new();

    let my_bool_handle = my_bool_container.insert(false);

    assert_eq!(*my_bool_container.get(my_bool_handle).unwrap(), false);

    my_bool_container.set(my_bool_handle, true);

    assert_eq!(*my_bool_container.get(my_bool_handle).unwrap(), true);
  
}
