use crate::extensions::NoGenericArgsGenericType;
use crate::ids::GenericTypeId;

#[derive(Default)]
pub struct DojoType {}
impl NoGenericArgsGenericType for DojoType {
    const ID: GenericTypeId = GenericTypeId::new_inline("Dojo");
    const STORABLE: bool = true;
    const DUPLICATABLE: bool = false;
    const DROPPABLE: bool = false;
    const ZERO_SIZED: bool = true;
}
