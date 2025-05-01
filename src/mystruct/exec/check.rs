use vstd::prelude::*;
use crate::mystruct::trusted::exec_types;
use crate::mystruct::trusted::spec_types;


verus!{

/*
 * checks exec_type::MyStruct
 * notice that well_formed can be inferred automatically by verus
 * since we assumed our field() ensures
 *      match field@ {
 *          Some(x) => self@.field == Some(x),
 *          None => self@.field == None::<bool>,
 *      }
 */
pub fn check_mystruct(mystruct: &exec_types::MyStruct) -> (res: bool)
    ensures
        mystruct@.well_formed() == res
{
    if let Some(f) = mystruct.field() {
        let mut result: bool = true;
        let mut idx: usize = 0;
        while idx < f.len()
            invariant
                idx <= f.len(),
                result == forall |i: int| 0 <= i < idx ==> spec_types::state_validation(f[i]@),
        {
            if exec_types::state_validation(&f[idx]) == false {
                result = false;
            }
            idx += 1;
        }
        // 1. assert result is correct for f
        assert(result == (forall |i: int| 0 <= i < f.len() ==> spec_types::state_validation(f[i]@)));
        // 2. assert f[i]@ is equal to mystruct@.field.get_Some_0()[i] for all i
        assert(forall |i: int| 0 <= i < mystruct@.field.get_Some_0().len() ==> (mystruct@.field.get_Some_0()[i] == f[i]@));
        // 3. assert f length is equal to original
        assert(mystruct@.field.get_Some_0().len() == f.len());
        // 4. assert again f[i]@ is equal to mystruct@.field.get_Some_0()[i] for all i with len substituted
        assert(forall |i: int| 0 <= i < f.len() ==> (mystruct@.field.get_Some_0()[i] == f[i]@));
        assert(forall |i: int| 0 <= i < f.len() ==> (
            spec_types::state_validation(f[i]@)
            == spec_types::state_validation(mystruct@.field.get_Some_0()[i])
        ));
        // 5. substitute f[i]@ with mystruct@.field.get_Some_0()[i] into 1. since by 2. and 4. f[i]@ is equal to mystruct@.field.get_Some_0()[i]
        // assert(result == (forall |i: int| 0 <= i < f.len() ==> spec_types::state_validation(mystruct@.field.get_Some_0()[i])));
        result
    } else {
        true
    }
}

}