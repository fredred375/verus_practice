use deps_hack;
use vstd::prelude::*;
use crate::mystruct::trusted::spec_types;

verus!{

// wrapper around the deps_hack type, used to convert to the proved type
#[verifier(external_body)]
pub struct MyStruct {
    inner: deps_hack::MyStruct
}

// view, so that the @ operator can be used to convert to spec_types::MyStructView
// while reasoning in ghost code
impl View for MyStruct {
    type V = spec_types::MyStructView;
    spec fn view(&self) -> spec_types::MyStructView;
}

// not verified, does conversion from/to deps_hack type to exec type
#[verifier(external)]
impl MyStruct {
    pub fn from_hack(inner: deps_hack::MyStruct) -> MyStruct {
        MyStruct { inner }
    }
    //not used in this example, but can be useful for other cases
    pub fn into_hack(self) -> deps_hack::MyStruct {
        self.inner
    }
}

impl MyStruct {
    // get function for exec_type::MyStruct
    // the postconditions are the starting point for reasoning on this type
    #[verifier(external_body)]
    pub fn field(&self) -> (field: Option<bool>)
        ensures
            match field@ {
                Some(x) => self@.field == Some(x),
                None => self@.field == None::<bool>,
            }
    {
        self.inner.field.clone()
    }
}

}
