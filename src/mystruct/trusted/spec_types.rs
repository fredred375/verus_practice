use vstd::prelude::*;

verus!{

pub struct MyStructView {
    pub field: Option<Seq<bool>>
}

impl MyStructView {
    pub open spec fn well_formed(&self) -> bool {
        self.field.is_Some() ==> (forall |i: int| 0 <= i < self.field.get_Some_0().len() ==> self.field.get_Some_0()[i])
    }
}

pub open spec fn state_validation(b: bool) -> bool {
    b
}

}