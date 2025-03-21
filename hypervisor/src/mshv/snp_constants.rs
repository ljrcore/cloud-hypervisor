// SPDX-License-Identifier: Apache-2.0 OR BSD-3-Clause
//
// Copyright © 2020, Microsoft Corporation
//

// Reference: https://www.amd.com/content/dam/amd/en/documents/epyc-technical-docs/specifications/56860.pdf
// Chapter 10
pub const SIG_R_COMPONENT_SIZE_IN_BYTES: usize = 72;
pub const SIG_R_AND_S_COMPONENT_SIZE_IN_BYTES: usize = 144;
pub const ECDSA_CURVE_ID_SIZE_IN_BYTES: usize = 4;
pub const ECDSA_SIG_X_COMPONENT_SIZE_IN_BYTES: usize = 72;
pub const ECDSA_SIG_Y_COMPONENT_SIZE_IN_BYTES: usize = 72;
pub const ECDSA_SIG_X_COMPONENT_START: usize = ECDSA_CURVE_ID_SIZE_IN_BYTES;
pub const ECDSA_SIG_X_COMPONENT_END: usize =
    ECDSA_SIG_X_COMPONENT_START + ECDSA_SIG_X_COMPONENT_SIZE_IN_BYTES;
pub const ECDSA_SIG_Y_COMPONENT_START: usize = ECDSA_SIG_X_COMPONENT_END;
pub const ECDSA_SIG_Y_COMPONENT_END: usize =
    ECDSA_SIG_X_COMPONENT_END + ECDSA_SIG_Y_COMPONENT_SIZE_IN_BYTES;
