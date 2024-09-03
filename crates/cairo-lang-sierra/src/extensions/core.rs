use super::ap_tracking::ApTrackingLibfunc;
use super::array::{ArrayLibfunc, ArrayType};
use super::bitwise::BitwiseType;
use super::boolean::BoolLibfunc;
use super::bounded_int::{BoundedIntLibfunc, BoundedIntType};
use super::branch_align::BranchAlignLibfunc;
use super::bytes31::{Bytes31Libfunc, Bytes31Type};
use super::casts::CastLibfunc;
use super::circuit::{CircuitLibFunc, CircuitType};
use super::const_type::{ConstLibfunc, ConstType};
use super::coupon::{CouponLibfunc, CouponType};
use super::debug::DebugLibfunc;
use super::drop::DropLibfunc;
use super::duplicate::DupLibfunc;
use super::ec::{EcLibfunc, EcOpType, EcPointType, EcStateType};
use super::enm::{EnumLibfunc, EnumType};
use super::felt252_dict::{
    Felt252DictEntryLibfunc, Felt252DictEntryType, Felt252DictLibfunc, Felt252DictType,
};
use super::function_call::CouponCallLibfunc;
use super::gas::BuiltinCostsType;
use super::int::signed::{
    Sint16Libfunc, Sint16Type, Sint32Libfunc, Sint32Type, Sint64Libfunc, Sint64Type, Sint8Libfunc,
    Sint8Type,
};
use super::int::signed128::{Sint128Libfunc, Sint128Type};
use super::int::unsigned::{
    Uint16Libfunc, Uint16Type, Uint32Libfunc, Uint32Type, Uint64Libfunc, Uint64Type, Uint8Libfunc,
    Uint8Type,
};
use super::int::unsigned128::{U128MulGuaranteeType, Uint128Libfunc, Uint128Type};
use super::int::unsigned256::Uint256Libfunc;
use super::int::unsigned512::Uint512Libfunc;
use super::modules::boxing::{BoxLibfunc, BoxType};
use super::modules::felt252::{Felt252Libfunc, Felt252Type};
use super::modules::function_call::FunctionCallLibfunc;
use super::modules::gas::{GasBuiltinType, GasLibfunc};
use super::modules::mem::MemLibfunc;
use super::modules::non_zero::{NonZeroType, UnwrapNonZeroLibfunc};
use super::modules::unconditional_jump::UnconditionalJumpLibfunc;
use super::nullable::{NullableLibfunc, NullableType};
use super::pedersen::{PedersenLibfunc, PedersenType};
use super::poseidon::{PoseidonLibfunc, PoseidonType};
use super::range::{IntRangeLibfunc, IntRangeType};
use super::range_check::{RangeCheck96Type, RangeCheckType};
use super::segment_arena::SegmentArenaType;
use super::snapshot::{SnapshotTakeLibfunc, SnapshotType};
use super::span::SpanType;
use super::squashed_felt252_dict::SquashedFelt252DictType;
use super::starknet::{StarkNetLibfunc, StarkNetType};
use super::structure::{StructLibfunc, StructType};
use super::uninitialized::UninitializedType;
use crate::{define_libfunc_hierarchy, define_type_hierarchy};

define_type_hierarchy! {
    pub enum CoreType {
        Array(ArrayType),
        Coupon(CouponType),
        Bitwise(BitwiseType),
        Box(BoxType),
        Circuit(CircuitType),
        Const(ConstType),
        EcOp(EcOpType),
        EcPoint(EcPointType),
        EcState(EcStateType),
        Felt252(Felt252Type),
        GasBuiltin(GasBuiltinType),
        IntRange(IntRangeType),
        BuiltinCosts(BuiltinCostsType),
        Uint8(Uint8Type),
        Uint16(Uint16Type),
        Uint32(Uint32Type),
        Uint64(Uint64Type),
        Uint128(Uint128Type),
        Uint128MulGuarantee(U128MulGuaranteeType),
        Sint8(Sint8Type),
        Sint16(Sint16Type),
        Sint32(Sint32Type),
        Sint64(Sint64Type),
        Sint128(Sint128Type),
        NonZero(NonZeroType),
        Nullable(NullableType),
        RangeCheck(RangeCheckType),
        RangeCheck96(RangeCheck96Type),
        Uninitialized(UninitializedType),
        Enum(EnumType),
        Struct(StructType),
        Felt252Dict(Felt252DictType),
        Felt252DictEntry(Felt252DictEntryType),
        SquashedFelt252Dict(SquashedFelt252DictType),
        Pedersen(PedersenType),
        Poseidon(PoseidonType),
        Span(SpanType),
        StarkNet(StarkNetType),
        SegmentArena(SegmentArenaType),
        Snapshot(SnapshotType),
        Bytes31(Bytes31Type),
        BoundedInt(BoundedIntType),
    }, CoreTypeConcrete
}

impl std::fmt::Debug for CoreType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variant_name = match self {
            CoreType::Array(_) => "Array",
            CoreType::Coupon(_) => "Coupon",
            CoreType::Bitwise(_) => "Bitwise",
            CoreType::Box(_) => "Box",
            CoreType::Circuit(_) => "Circuit",
            CoreType::Const(_) => "Const",
            CoreType::EcOp(_) => "EcOp",
            CoreType::EcPoint(_) => "EcPoint",
            CoreType::EcState(_) => "EcState",
            CoreType::Felt252(_) => "Felt252",
            CoreType::GasBuiltin(_) => "GasBuiltin",
            CoreType::BuiltinCosts(_) => "BuiltinCosts",
            CoreType::Uint8(_) => "Uint8",
            CoreType::Uint16(_) => "Uint16",
            CoreType::Uint32(_) => "Uint32",
            CoreType::Uint64(_) => "Uint64",
            CoreType::Uint128(_) => "Uint128",
            CoreType::Uint128MulGuarantee(_) => "Uint128MulGuarantee",
            CoreType::Sint8(_) => "Sint8",
            CoreType::Sint16(_) => "Sint16",
            CoreType::Sint32(_) => "Sint32",
            CoreType::Sint64(_) => "Sint64",
            CoreType::Sint128(_) => "Sint128",
            CoreType::NonZero(_) => "NonZero",
            CoreType::Nullable(_) => "Nullable",
            CoreType::RangeCheck(_) => "RangeCheck",
            CoreType::RangeCheck96(_) => "RangeCheck96",
            CoreType::Uninitialized(_) => "Uninitialized",
            CoreType::Enum(_) => "Enum",
            CoreType::Struct(_) => "Struct",
            CoreType::Felt252Dict(_) => "Felt252Dict",
            CoreType::Felt252DictEntry(_) => "Felt252DictEntry",
            CoreType::SquashedFelt252Dict(_) => "SquashedFelt252Dict",
            CoreType::Pedersen(_) => "Pedersen",
            CoreType::Poseidon(_) => "Poseidon",
            CoreType::Span(_) => "Span",
            CoreType::StarkNet(_) => "StarkNet",
            CoreType::SegmentArena(_) => "SegmentArena",
            CoreType::Snapshot(_) => "Snapshot",
            CoreType::Bytes31(_) => "Bytes31",
            CoreType::BoundedInt(_) => "BoundedInt",
        };
        write!(f, "{}", variant_name)
    }
}

define_libfunc_hierarchy! {
    pub enum CoreLibfunc {
        ApTracking(ApTrackingLibfunc),
        Array(ArrayLibfunc),
        BranchAlign(BranchAlignLibfunc),
        Bool(BoolLibfunc),
        Box(BoxLibfunc),
        Cast(CastLibfunc),
        Circuit(CircuitLibFunc),
        Coupon(CouponLibfunc),
        CouponCall(CouponCallLibfunc),
        Drop(DropLibfunc),
        Dup(DupLibfunc),
        Ec(EcLibfunc),
        Felt252(Felt252Libfunc),
        Const(ConstLibfunc),
        FunctionCall(FunctionCallLibfunc),
        Gas(GasLibfunc),
        IntRange(IntRangeLibfunc),
        Uint8(Uint8Libfunc),
        Uint16(Uint16Libfunc),
        Uint32(Uint32Libfunc),
        Uint64(Uint64Libfunc),
        Uint128(Uint128Libfunc),
        Uint256(Uint256Libfunc),
        Uint512(Uint512Libfunc),
        Sint8(Sint8Libfunc),
        Sint16(Sint16Libfunc),
        Sint32(Sint32Libfunc),
        Sint64(Sint64Libfunc),
        Sint128(Sint128Libfunc),
        Mem(MemLibfunc),
        Nullable(NullableLibfunc),
        UnwrapNonZero(UnwrapNonZeroLibfunc),
        UnconditionalJump(UnconditionalJumpLibfunc),
        Enum(EnumLibfunc),
        Struct(StructLibfunc),
        Felt252Dict(Felt252DictLibfunc),
        Felt252DictEntry(Felt252DictEntryLibfunc),
        Pedersen(PedersenLibfunc),
        Poseidon(PoseidonLibfunc),
        StarkNet(StarkNetLibfunc),
        Debug(DebugLibfunc),
        SnapshotTake(SnapshotTakeLibfunc),
        Bytes31(Bytes31Libfunc),
        BoundedInt(BoundedIntLibfunc),
    }, CoreConcreteLibfunc
}

impl std::fmt::Debug for CoreConcreteLibfunc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variant_name = match self {
            CoreConcreteLibfunc::ApTracking(_) => "ApTracking",
            CoreConcreteLibfunc::Array(_) => "Array",
            CoreConcreteLibfunc::BranchAlign(_) => "BranchAlign",
            CoreConcreteLibfunc::Bool(_) => "Bool",
            CoreConcreteLibfunc::Box(_) => "Box",
            CoreConcreteLibfunc::Cast(_) => "Cast",
            CoreConcreteLibfunc::Circuit(_) => "Circuit",
            CoreConcreteLibfunc::Coupon(_) => "Coupon",
            CoreConcreteLibfunc::CouponCall(_) => "CouponCall",
            CoreConcreteLibfunc::Drop(_) => "Drop",
            CoreConcreteLibfunc::Dup(_) => "Dup",
            CoreConcreteLibfunc::Ec(_) => "Ec",
            CoreConcreteLibfunc::Felt252(_) => "Felt252",
            CoreConcreteLibfunc::Const(_) => "Const",
            CoreConcreteLibfunc::FunctionCall(_) => "FunctionCall",
            CoreConcreteLibfunc::Gas(_) => "Gas",
            CoreConcreteLibfunc::Uint8(_) => "Uint8",
            CoreConcreteLibfunc::Uint16(_) => "Uint16",
            CoreConcreteLibfunc::Uint32(_) => "Uint32",
            CoreConcreteLibfunc::Uint64(_) => "Uint64",
            CoreConcreteLibfunc::Uint128(_) => "Uint128",
            CoreConcreteLibfunc::Uint256(_) => "Uint256",
            CoreConcreteLibfunc::Uint512(_) => "Uint512",
            CoreConcreteLibfunc::Sint8(_) => "Sint8",
            CoreConcreteLibfunc::Sint16(_) => "Sint16",
            CoreConcreteLibfunc::Sint32(_) => "Sint32",
            CoreConcreteLibfunc::Sint64(_) => "Sint64",
            CoreConcreteLibfunc::Sint128(_) => "Sint128",
            CoreConcreteLibfunc::Mem(_) => "Mem",
            CoreConcreteLibfunc::Nullable(_) => "Nullable",
            CoreConcreteLibfunc::UnwrapNonZero(_) => "UnwrapNonZero",
            CoreConcreteLibfunc::UnconditionalJump(_) => "UnconditionalJump",
            CoreConcreteLibfunc::Enum(_) => "Enum",
            CoreConcreteLibfunc::Struct(_) => "Struct",
            CoreConcreteLibfunc::Felt252Dict(_) => "Felt252Dict",
            CoreConcreteLibfunc::Felt252DictEntry(_) => "Felt252DictEntry",
            CoreConcreteLibfunc::Pedersen(_) => "Pedersen",
            CoreConcreteLibfunc::Poseidon(_) => "Poseidon",
            CoreConcreteLibfunc::StarkNet(_) => "StarkNet",
            CoreConcreteLibfunc::Debug(_) => "Debug",
            CoreConcreteLibfunc::SnapshotTake(_) => "SnapshotTake",
            CoreConcreteLibfunc::Bytes31(_) => "Bytes31",
            CoreConcreteLibfunc::BoundedInt(_) => "BoundedInt",
        };
        write!(f, "{}", variant_name)
    }
}
