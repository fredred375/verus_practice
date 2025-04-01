use vstd::prelude::*;

verus!{

pub struct MyStructView {
    pub field: Option<bool>
}

impl MyStructView {
    pub open spec fn well_formed(&self) -> bool {
        self.field.is_some()
    }
}

}