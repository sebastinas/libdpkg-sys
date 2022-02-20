// Copyright 2022 Sebastian Ramacher
// SPDX-License-Identifier: LGPL-3.0-or-later

//! Bindings for libdpkg
//!
//! This module provides access to parts of the public API of `libdpkg`.
//!
//! For more information on the use of these function, please see the documentation of [libdpkg](https://www.dpkg.org/doc/libdpkg/index.html).

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod test {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn version() {
        let version_base = CString::new("0.1").unwrap();
        let revision1 = CString::new("1").unwrap();
        let revision2 = CString::new("2").unwrap();
        let version1 = dpkg_version {
            epoch: 0,
            version: version_base.as_ptr(),
            revision: revision1.as_ptr(),
        };
        let version2 = dpkg_version {
            epoch: 0,
            version: version_base.as_ptr(),
            revision: revision2.as_ptr(),
        };

        assert!(unsafe { dpkg_version_is_informative(&version1) });
        assert!(unsafe { dpkg_version_is_informative(&version2) });
        assert!(unsafe { dpkg_version_compare(&version1, &version2) } < 0);
        assert!(unsafe {
            dpkg_version_relate(&version1, dpkg_relation::DPKG_RELATION_LT, &version2)
        });
        assert!(unsafe {
            dpkg_version_relate(&version1, dpkg_relation::DPKG_RELATION_LE, &version2)
        });
        assert!(!unsafe {
            dpkg_version_relate(&version1, dpkg_relation::DPKG_RELATION_GE, &version2)
        });
        assert!(!unsafe {
            dpkg_version_relate(&version1, dpkg_relation::DPKG_RELATION_GT, &version2)
        });
        assert!(!unsafe {
            dpkg_version_relate(&version1, dpkg_relation::DPKG_RELATION_EQ, &version2)
        });
    }
}
