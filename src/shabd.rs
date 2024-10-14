use crate::VarnList;
use crate::Varn;

pub struct Akshar {
    pub akshar: VarnList,
    pub matra: u32,
}

impl Akshar {
    pub fn new() -> Self {
        Akshar {
                akshar: VarnList::new(),
                matra : 0,
        }
    }
}

pub struct Shabd {
    pub base: VarnList,
    pub akshars: Vec<Akshar>,
    pub matra: u32,
}

impl Shabd {
    pub fn new() -> Self {
        Shabd {
                base: VarnList::new(),
                akshars: Vec::new(),
                matra: 0,
        }
    }

    pub fn get_matra(&mut self) {
        self.matra = 0;
        for akshar in self.akshars.iter() {
            self.matra += akshar.matra;
        }
    }

    pub fn new_from_list(varnlist: VarnList) -> Self {
        Shabd {
                base: varnlist,
                akshars: Vec::new(),
                matra: 0,
        }
    }

    pub fn process_akshars(&mut self, varn: Varn) {
        let mut a = Akshar::new();
        a.matra = varn.get_matra();
        a.akshar.push(varn);
        self.akshars.push(a);
    }

    pub fn process_chihn(&mut self, varn: Varn) {
        let mut a = Akshar::new();
        a.matra = varn.get_matra();
        while let Some(v) = self.base.pop() {
            if v.is_chihn() {
                a.akshar.push(v);
                continue;
            }
            else {
                a.akshar.push(v);
                a.akshar.reverse();
                break;
            }
        }
        a.akshar.push(varn);
        self.akshars.push(a);
    }

    pub fn process_halant(&mut self, varn: Varn) {
        let mut a = Akshar::new();
        if let Some(varn2) = self.base.pop() {
            a.akshar.push(varn);
            a.akshar.push(varn2);
        }
        /*
         * Next can be Vyanjan, Swar, Chihn, Halant or Others
         * Keep on popping till we keep on getting Halant followed by Vyanjan
         */
        loop {
            if let Some(varn3) = self.base.pop() {
                if varn3.is_swar() || varn3.is_vyanjan() || varn3.is_avgrah() {
                    a.matra = 2;
                    a.akshar.push(varn3);
                    a.akshar.reverse();
                    self.akshars.push(a);
                    break;
                } else if varn3.is_chihn() {
                    if let Some(varn4) = self.base.pop() {
                        a.matra = 2;
                        a.akshar.push(varn3);
                        a.akshar.push(varn4);
                        a.akshar.reverse();
                        self.akshars.push(a);
                        break;
                    }
                } else if varn3.is_halant() {
                    if let Some(varn4) = self.base.pop() {
                        a.akshar.push(varn3);
                        a.akshar.push(varn4);
                    }
                }
            } else {
                    a.matra = 0;
                    a.akshar.reverse();
                    self.akshars.push(a);
                    break;
            }
        }

    }

    pub fn make_akshars(&mut self) {
        let cp = self.base.copy();
        while let Some(x) = self.base.pop() {
            if x.is_swar() || x.is_vyanjan() || x.is_avgrah() {
                self.process_akshars(x);
            } else if x.is_chihn() && !x.is_halant() {
                self.process_chihn(x);
            } else if x.is_halant() {
                self.process_halant(x);
            }
        }
        self.akshars.reverse();
        self.base = cp.copy();
    }

    pub fn from_str(input: &str) -> Self {
        let varnlist = VarnList::from_str(input);
        let mut shabd = Shabd::new_from_list(varnlist);
        shabd.make_akshars();
        shabd.get_matra();

        shabd
    }
}
