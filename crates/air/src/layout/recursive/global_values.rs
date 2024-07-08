use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use starknet_core::serde::unsigned_field_element::UfeHex;
use starknet_crypto::Felt;

#[serde_as]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EcPoint {
    #[serde_as(as = "UfeHex")]
    x: Felt,
    #[serde_as(as = "UfeHex")]
    y: Felt,
}

// Accumulation of member expressions for auto generated composition polynomial code.
#[serde_as]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GlobalValues {
    // Public input.
    #[serde_as(as = "UfeHex")]
    trace_length: Felt,
    #[serde_as(as = "UfeHex")]
    initial_pc: Felt,
    #[serde_as(as = "UfeHex")]
    final_pc: Felt,
    #[serde_as(as = "UfeHex")]
    initial_ap: Felt,
    #[serde_as(as = "UfeHex")]
    final_ap: Felt,
    #[serde_as(as = "UfeHex")]
    initial_pedersen_addr: Felt,
    #[serde_as(as = "UfeHex")]
    initial_range_check_addr: Felt,
    #[serde_as(as = "UfeHex")]
    initial_bitwise_addr: Felt,
    #[serde_as(as = "UfeHex")]
    range_check_min: Felt,
    #[serde_as(as = "UfeHex")]
    range_check_max: Felt,
    // Constants.
    #[serde_as(as = "UfeHex")]
    offset_size: Felt,
    #[serde_as(as = "UfeHex")]
    half_offset_size: Felt,
    pedersen_shift_point: EcPoint,
    // Periodic columns.
    #[serde_as(as = "UfeHex")]
    pedersen_points_x: Felt,
    #[serde_as(as = "UfeHex")]
    pedersen_points_y: Felt,
    // Interaction elements.
    #[serde_as(as = "UfeHex")]
    memory_multi_column_perm_perm_interaction_elm: Felt,
    #[serde_as(as = "UfeHex")]
    memory_multi_column_perm_hash_interaction_elm0: Felt,
    #[serde_as(as = "UfeHex")]
    range_check16_perm_interaction_elm: Felt,
    #[serde_as(as = "UfeHex")]
    diluted_check_permutation_interaction_elm: Felt,
    #[serde_as(as = "UfeHex")]
    diluted_check_interaction_z: Felt,
    #[serde_as(as = "UfeHex")]
    diluted_check_interaction_alpha: Felt,
    // Permutation products.
    #[serde_as(as = "UfeHex")]
    memory_multi_column_perm_perm_public_memory_prod: Felt,
    #[serde_as(as = "UfeHex")]
    range_check16_perm_public_memory_prod: Felt,
    #[serde_as(as = "UfeHex")]
    diluted_check_first_elm: Felt,
    #[serde_as(as = "UfeHex")]
    diluted_check_permutation_public_memory_prod: Felt,
    #[serde_as(as = "UfeHex")]
    diluted_check_final_cum_val: Felt,
}

// Elements that are sent from the prover after the commitment on the original trace.
// Used for components after the first interaction, e.g., memory and range check.
#[serde_as]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct InteractionElements {
    #[serde_as(as = "UfeHex")]
    pub memory_multi_column_perm_perm_interaction_elm: Felt,
    #[serde_as(as = "UfeHex")]
    pub memory_multi_column_perm_hash_interaction_elm0: Felt,
    #[serde_as(as = "UfeHex")]
    pub range_check16_perm_interaction_elm: Felt,
    #[serde_as(as = "UfeHex")]
    pub diluted_check_permutation_interaction_elm: Felt,
    #[serde_as(as = "UfeHex")]
    pub diluted_check_interaction_z: Felt,
    #[serde_as(as = "UfeHex")]
    pub diluted_check_interaction_alpha: Felt,
}
