/// Helper for `wrong-call-return-type-due-to-generic-arg.rs`: the diagnostic help tested by that
/// file shouldn't label calls produced by external macros. In this case, we shouldn't get a label
/// saying that the argument to `identity` influences its return type, since there's not much a user
/// can do with that information.
#[macro_export]
macro_rules! with_wrong_type_due_to_generic_arg {
    () => {{
        let _: u8 = core::convert::identity(());
    }}
}
