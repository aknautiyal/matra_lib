use matra_lib::{Varn};

struct Varnmala {
    varns: Vec<Varn>,
}

impl Varnmala {
    pub fn new() -> Self {
        Varnmala { varns: Vec::new() }
    }

    pub fn add_varn(&mut self, new_varn: Varn) {
        self.varns.push(new_varn);
    }

    pub fn initialize_varns(&mut self) {
        for code_point in 0x0900..=0x0954 {
            let varn = Varn::from_scalar(code_point);
            self.add_varn(varn);
        }
    }

    pub fn print_all_varns(&self) {
        for varn in &self.varns {
            varn.print();
        }
    }
}

#[test]
fn test_varnmala_initialization() {
    let mut varnmala = Varnmala::new();
    varnmala.initialize_varns();
    assert_eq!(varnmala.varns.len(), 85);  // Adjust test for expected length
}

#[test]
fn test_varnmala_print() {
    let mut varnmala = Varnmala::new();
    varnmala.initialize_varns();
    varnmala.print_all_varns();  // This will print, useful for manual testing
}

