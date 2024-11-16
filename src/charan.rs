use crate::Shabd;
pub struct Charan {
    pub shabds: Vec<Shabd>,
    pub matra: u32,
}

impl Charan {
    pub fn from_str(sentence: &str) -> Self {
        let mut shabds = Vec::new();
        let mut matra : u32 = 0;
        for word in sentence.split_whitespace() {
            let shabd = Shabd::from_str(word);
            matra = matra + shabd.matra;
            shabds.push(shabd);
        }

        Charan { shabds, matra }
    }

    pub fn analysis(&self) -> String {
        let mut result = String::new();
        for s in &self.shabds {
            result.push_str(&format!("{} ", s.matra));
        }
        result.trim_end().to_string()
    }
}
