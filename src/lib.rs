mod varn;
mod shabd;
pub use varn::{Varn, VarnType, VarnList};
pub use shabd::{Akshar, Shabd};

#[cfg(test)]
mod tests {
    use crate::{Varn, Shabd};

    #[test]
    fn test_varn_initialization() {
        let varn = Varn::from_char('अ');
        assert_eq!(varn.get_symbol(), 'अ');
        assert_eq!(varn.get_scalar(), 'अ' as u32);
        assert!(varn.is_swar());
    }

    #[test]
    fn test_api_exposure() {
        let varn1 = Varn::from_char('क');
        assert_eq!(varn1.get_symbol(), 'क');
        assert!(varn1.is_vyanjan());

        let varn2 = Varn::from_scalar(0x0905);
        assert_eq!(varn2.get_symbol(), 'अ');

        let shabd = Shabd::from_str("राम");
        assert_eq!(shabd.base.varns.len(), 3);
        assert_eq!(shabd.base.varns[0].get_symbol(), 'र');
    }
}
