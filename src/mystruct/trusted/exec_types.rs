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
    pub fn field(&self) -> (field: Option<Vec<MyBool>>)
        ensures
            field.is_Some() == self@.field.is_Some(),
            field.is_Some() ==> self@.field.get_Some_0() == field.get_Some_0()@.map(|i: int, x: MyBool| x@),
    {
        match &self.inner.field {
            Some(f) => {
                Some(f.iter().map(|x| MyBool::from_hack(x.clone())).collect())
            }
            None => None,
        }
    }
}

#[verifier(external_body)]
pub struct MyBool {
    inner: deps_hack::MyBool
}

impl View for MyBool {
    type V = bool;
    open spec fn view(&self) -> bool;
}

#[verifier(external)]
impl MyBool {
    pub fn from_hack(inner: deps_hack::MyBool) -> MyBool {
        MyBool { inner }
    }
    //not used in this example, but can be useful for other cases
    pub fn into_hack(self) -> deps_hack::MyBool {
        self.inner
    }
}

impl MyBool {
    // get function for exec_type::MyBool
    // the postconditions are the starting point for reasoning on this type
    #[verifier(external_body)]
    pub fn b(&self) -> (b: bool)
        ensures
            self@ == b,
    {
        self.inner.b != 0
    }
}

// pub fn state_validation(mybool: &MyBool) -> (res: bool)
//     ensures
//         spec_types::state_validation(mybool@) == res,
// {
//     mybool.b()
// }

pub fn state_validation(mybool: bool) -> (res: bool)
    ensures
        spec_types::state_validation(mybool) == res,
{
    mybool
}

#[verifier(external_body)]
pub fn map_vec(
    vec: Vec<MyBool>,
) -> (res: Vec<bool>)
    ensures
        res@ == vec@.map(|i: int, b: MyBool| b@),
{
    vec.iter().map(|x| x.b()).collect()
}

}
