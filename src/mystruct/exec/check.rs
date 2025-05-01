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
// #[verifier::loop_isolation(false)]
pub fn check_mystruct(mystruct: &exec_types::MyStruct) -> (res: bool)
    ensures
        mystruct@.well_formed() == res
{
    if let Some(f) = mystruct.field() {
        let mut result: bool = true;
        let mut idx: usize = 0;
        let ghost mut f_view: Seq<spec_types::MyNumView> = Seq::new(f.len() as nat, |i: int| f[i]@);
        assert(f@.map_values(|n: exec_types::MyNum| n@) == f_view);
        while idx < f.len()
            invariant
                0 <= idx <= f_view.len(),
                f@.map_values(|n: exec_types::MyNum| n@) == f_view,
                result == forall |i: int| 0 <= i < idx ==> f_view[i].state_validation(),
        {
            let exec_sv = f[idx].state_validation();
            // assert(exec_sv == f[idx as int]@.state_validation());
            assert(exec_sv == f_view[idx as int].state_validation());
            // assert(
            //     ((forall |i: int| 0 <= i < idx ==> spec_types::state_validation(f_view[i])) && exec_sv)
            //     ==
            //     forall |i: int| 0 <= i < (idx + 1) ==> spec_types::state_validation(f_view[i])
            // );
            // assert((result && exec_sv) == (forall |i: int| 0 <= i < (idx + 1) ==> spec_types::state_validation(f_view[i])));
            result = result && exec_sv;
            idx += 1;
        }
        result
    } else {
        true
    }
}

}

// 1. assert result is correct for f
        // assert(result == (forall |i: int| 0 <= i < f.len() ==> spec_types::state_validation(&f[i]@)));
        // // 2. assert f[i]@ is equal to mystruct@.field.get_Some_0()[i] for all i
        // assert(forall |i: int| 0 <= i < mystruct@.field.get_Some_0().len() ==> (mystruct@.field.get_Some_0()[i] == f[i]@));
        // // 3. assert f length is equal to original
        // assert(mystruct@.field.get_Some_0().len() == f.len());
        // // 4. assert again f[i]@ is equal to mystruct@.field.get_Some_0()[i] for all i with len substituted
        // assert(forall |i: int| 0 <= i < f.len() ==> (mystruct@.field.get_Some_0()[i] == f[i]@));
        // assert(forall |i: int| 0 <= i < f.len() ==> (
        //     spec_types::state_validation(&f[i]@)
        //     == spec_types::state_validation(&mystruct@.field.get_Some_0()[i])
        // ));
        // 5. substitute f[i]@ with mystruct@.field.get_Some_0()[i] into 1. since by 2. and 4. f[i]@ is equal to mystruct@.field.get_Some_0()[i]
        // assert(result == (forall |i: int| 0 <= i < f.len() ==> spec_types::state_validation(mystruct@.field.get_Some_0()[i])));