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

    pub fn analysis(&self) -> (String, String) {
        let mut result_akshar = String::new();
        let mut result_matra = String::new();
        let mut akshar_count = 0;

        for s in &self.shabds {

            result_akshar.push_str(&format!("{{"));
            result_matra.push_str(&format!("{{"));
            let mut c = 0;
            for a in &s.akshars {
                result_akshar.push_str(&format!("{}", a.akshar.to_str()));
                result_matra.push_str(&format!("{}", a.matra));
                c = c + 1;
                if c != s.akshars.len() {
                    result_akshar.push_str(&format!(","));
                    result_matra.push_str(&format!(","));
                }
            }
            result_akshar.push_str(&format!("}} "));
            result_matra.push_str(&format!("}} "));

            akshar_count = akshar_count + s.akshar_count;
        }
        result_akshar.push_str(&format!("= {} ", akshar_count));
        result_matra.push_str(&format!("= {} ", self.matra));

        (
            result_akshar.trim_end().to_string(),
            result_matra.trim_end().to_string(),
        )
    }
}
