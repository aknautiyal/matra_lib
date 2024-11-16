mod varn;
mod shabd;
mod charan;
pub use varn::{Varn, VarnType, VarnList};
pub use shabd::{Akshar, Shabd};
pub use charan::Charan;

#[cfg(test)]
mod tests {
    use crate::{Varn, Shabd, Charan};

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

        let sentence = "सीता राम राधे श्याम";
        let charan = Charan::from_str(sentence);

        assert_eq!(charan.shabds.len(), 4);
        assert_eq!(charan.shabds[0].base.varns[0].get_symbol(), 'स');
        assert_eq!(charan.shabds[1].base.varns[0].get_symbol(), 'र');
        assert_eq!(charan.shabds[2].base.varns[0].get_symbol(), 'र');
        assert_eq!(charan.shabds[3].base.varns[0].get_symbol(), 'श');

    }

    #[test]
    fn test_charan_integration_analysis() {
        let input = "राम सीता"; //
        let charan = Charan::from_str(input);

        let expected_analysis = format!("{} {}", charan.shabds[0].matra, charan.shabds[1].matra);
        assert_eq!(charan.analysis(), expected_analysis);
    }
}
