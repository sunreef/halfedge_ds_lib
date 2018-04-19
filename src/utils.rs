use std::rc::Rc;
use std::cell::RefCell;

pub type Handle<T> = Rc<RefCell<T>>;

pub fn new_handle<T>(object: T) -> Handle<T> {
    Rc::new(RefCell::new(object))
}

macro_rules! get_element {
    ($parent: ident, $attribute: ident) => {{
        let r = $parent.borrow();
        match r.$attribute {
            Some(ref o) => Rc::clone(o),
            None => panic!("Error: Option returned None when trying to get attribute {} from object {}!", stringify!($attribute), stringify!($parent)),
        }
    }};
}
