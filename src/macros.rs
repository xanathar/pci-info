#[cfg(test)]
macro_rules! test_enumerator {
    ($enumeratorname:ident, $initializer:expr) => {
        paste::paste! {
            #[cfg(test)]
            mod [<$enumeratorname:snake _test>] {
                use super::*;
                use $crate::PciEnumerator;
                #[test]
                fn [<$enumeratorname:snake _enumeration_check>]() {
                    let enumerator = $initializer;
                    let res = enumerator.enumerate_pci().unwrap();

                    for r in res.iter() {
                        r.unwrap();
                    }
                }
            }
        }
    };
}

#[cfg(not(test))]
macro_rules! test_enumerator {
    ($enumeratorname:ident, $initializer:expr) => {};
}
