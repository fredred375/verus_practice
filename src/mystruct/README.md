# Folder structure

Code meant for verification is in `exec/`.
Code in `trusted/` are inherently trusted and not checked by verus, they are either spec functions or unverified exec functions.
As you can see, exec code in `trusted/exec_types.rs` are labled with `#[verifier(external_body)]`.