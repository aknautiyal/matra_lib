#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VarnType {
    SWAR,
    VYANJAN,
    CHIHN,
    HALANT,
    OTHERS,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Varn {
    symbol: char,
    scalar: u32,
    matra: u32,
    varn_type: VarnType,
}

impl Varn {
    pub fn print(&self) {
        println!("Scalar: U+{:04X} | Symbol: {} | Matra: {} | Type: {:?}", self.scalar, self.symbol, self.matra, self.varn_type);
    }

    pub fn set_matra(symbol: char) -> u32 {
       let matra : u32;
        match symbol {
            'आ'|'ई'|'ऊ'|'ए'|'ऐ'|'ओ'|'औ'|
            'ा'|'ी'|'ू'|'े'|'ै'|'ो'|'ौ'|'ं'|'ः'  => matra = 2,
            _ => matra = 1,
        };
        matra
    }

    pub fn set_type(symbol: char) -> VarnType {
        let vtype : VarnType;
        match symbol {
            'अ'|'आ'|'इ'|'ई'|'उ'|'ऊ'|'ए'|'ऐ'|'ओ'|'औ'|'ऋ' => vtype = VarnType::SWAR,

            'क'|'ख'|'ग'|'घ'|'ङ'|
            'च'|'छ'|'ज'|'झ'|'ञ'|
            'ट'|'ठ'|'ड'|'ढ'|'ण'|
            'त'|'थ'|'द'|'ध'|'न'|
            'प'|'फ'|'ब'|'भ'|'म'|
            'य'|'र'|'ल'|'व'|'श'|'स'|'ष'|'ह' => vtype = VarnType::VYANJAN,

            'ा'|'ि'|'ी'|'ु'|'ू'|'ो'|'ौ'|'े'|'ै'|'ृ'|'ः'|'ँ'|'ं'|'ऽ' => vtype = VarnType::CHIHN,
            '्' => vtype = VarnType::HALANT,

            _=> vtype = VarnType::OTHERS,

        };
        vtype
    }

    pub fn get_matra(&self) -> u32 {
        self.matra
    }

    pub fn get_type(&self) -> VarnType {
        self.varn_type
    }

    pub fn get_scalar(&self) -> u32 {
        self.scalar
    }

    pub fn get_symbol(&self) -> char {
        self.symbol
    }

    pub fn from_scalar(new_scalar: u32) -> Self {
        let sym = std::char::from_u32(new_scalar).unwrap();
        Self {
                symbol: sym,
                scalar: new_scalar,
                matra: Varn::set_matra(sym),
                varn_type: Varn::set_type(sym),
            }
    }

    pub fn from_char(sym: char) -> Self {
        let scalar_val : u32 = sym as u32;
        Self {
                symbol: sym,
                scalar: scalar_val,
                matra: Varn::set_matra(sym),
                varn_type: Varn::set_type(sym),
            }
    }

    pub fn is_avgrah(&self) -> bool {
        self.symbol == 'ऽ'
    }

    pub fn is_halant(&self) -> bool {
        self.varn_type == VarnType::HALANT
    }

    pub fn is_swar(&self) -> bool {
        self.varn_type == VarnType::SWAR
    }

    pub fn is_vyanjan(&self) -> bool {
        self.varn_type == VarnType::VYANJAN
    }

    pub fn is_chihn(&self) -> bool {
        self.varn_type == VarnType::CHIHN
    }
}

/* Tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_matra() {
        assert_eq!(Varn::set_matra('आ'), 2);
        assert_eq!(Varn::set_matra('क'), 1);
    }

    #[test]
    fn test_set_type() {
        assert_eq!(Varn::set_type('अ'), VarnType::SWAR);
        assert_eq!(Varn::set_type('क'), VarnType::VYANJAN);
        assert_eq!(Varn::set_type('ा'), VarnType::CHIHN);
        assert_eq!(Varn::set_type('्'), VarnType::HALANT);
        assert_eq!(Varn::set_type('X'), VarnType::OTHERS);
    }

    #[test]
    fn test_from_scalar() {
        let varn = Varn::from_scalar(0x0905);
        assert_eq!(varn.symbol, 'अ');
        assert_eq!(varn.scalar, 0x0905);
        assert_eq!(varn.matra, 1);
        assert_eq!(varn.varn_type, VarnType::SWAR);
    }

    #[test]
    fn test_from_char() {
        let varn = Varn::from_char('क');
        assert_eq!(varn.symbol, 'क');
        assert_eq!(varn.scalar, 'क' as u32);
        assert_eq!(varn.matra, 1);
        assert_eq!(varn.varn_type, VarnType::VYANJAN);
    }

    #[test]
    fn test_copy() {
        let varn1 = Varn::from_char('अ');
        let varn2 = varn1;
        assert_eq!(varn1, varn2);
        assert_eq!(varn1.symbol, varn2.symbol);
    }

    #[test]
    fn test_is_avgrah() {
        let varn1 = Varn::from_char('ऽ');
        let varn2 = Varn::from_char('क');
        assert!(varn1.is_avgrah());
        assert!(!varn2.is_avgrah());
    }

    #[test]
    fn test_is_halant() {
        let varn = Varn::from_char('्');
        assert!(varn.is_halant());
    }

    #[test]
    fn test_is_swar() {
        let varn = Varn::from_char('अ');
        assert!(varn.is_swar());
    }

    #[test]
    fn test_is_vyanjan() {
        let varn = Varn::from_char('क');
        assert!(varn.is_vyanjan());
    }

    #[test]
    fn test_is_chihn() {
        let varn = Varn::from_char('ा');
        assert!(varn.is_chihn());
    }
}