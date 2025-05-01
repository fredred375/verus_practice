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
    pub fn field(&self) -> (field: Option<Vec<MyNum>>)
        ensures
            field.is_Some() == self@.field.is_Some(),
            field.is_Some() ==> self@.field.get_Some_0() == field.get_Some_0()@.map(|i: int, x: MyNum| x@),
    {
        match &self.inner.field {
            Some(f) => {
                Some(f.iter().map(|x| MyNum::from_hack(x.clone())).collect())
            }
            None => None,
        }
    }
}

#[verifier(external_body)]
pub struct MyNum {
    inner: deps_hack::MyNum
}

impl View for MyNum {
    type V = i32;
    open spec fn view(&self) -> i32;
}

#[verifier(external)]
impl MyNum {
    pub fn from_hack(inner: deps_hack::MyNum) -> MyNum {
        MyNum { inner }
    }
    //not used in this example, but can be useful for other cases
    pub fn into_hack(self) -> deps_hack::MyNum {
        self.inner
    }
}

impl MyNum {
    // get function for exec_type::MyNum
    // the postconditions are the starting point for reasoning on this type
    #[verifier(external_body)]
    pub fn num(&self) -> (num: i32)
        ensures
            self@ == num,
    {
        self.inner.num
    }
}

// pub fn state_validation(mybool: &MyNum) -> (res: bool)
//     ensures
//         spec_types::state_validation(mybool@) == res,
// {
//     mybool.b()
// }

pub fn state_validation(mynum: i32) -> (res: bool)
    ensures
        spec_types::state_validation(mynum) == res,
{
    mynum != 0
}

#[verifier(external_body)]
pub fn map_vec(
    vec: Vec<MyNum>,
) -> (res: Vec<i32>)
    ensures
        res@ == vec@.map(|i: int, b: MyNum| b@),
{
    vec.iter().map(|x| x.num()).collect()
}

}
