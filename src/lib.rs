#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub use cuda_driver_sys::*;

include! {"./cupti.rs"}

#[cfg(test)]
mod tests {
    use crate as cupti;

    #[test]
    fn test_version() {
        let mut version = 0;
        let res = unsafe { cupti::cuptiGetVersion(&mut version) };
        eprintln!("CUpti version = {}", version);
        assert_eq!(res, cupti::CUptiResult::CUPTI_SUCCESS);
    }
}
