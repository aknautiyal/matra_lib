use matra_lib::{Varn, Shabd, Akshar};

#[test]
fn test_akshar_initialization() {
    let akshar = Akshar::new();
    assert_eq!(akshar.akshar.varns.len(), 0); // VarnList should be empty
    assert_eq!(akshar.matra, 0); // Matra should be initialized to 0
}

#[test]
fn test_shabd_initialization() {
    let shabd = Shabd::new();
    assert_eq!(shabd.base.varns.len(), 0); // VarnList in Shabd should be empty
    assert_eq!(shabd.akshars.len(), 0); // Akshars should be empty
    assert_eq!(shabd.matra, 0); // Matra should be 0
}

#[test]
fn test_shabd_process_akshars() {
    let mut shabd = Shabd::new();
    let varn = Varn::from_char('अ'); // Swar Varn
    shabd.process_akshars(varn);

    assert_eq!(shabd.akshars.len(), 1); // One Akshar should be created
    assert_eq!(shabd.akshars[0].akshar.varns.len(), 1); // Akshar contains one Varn
    assert_eq!(shabd.akshars[0].akshar.varns[0].get_symbol(), 'अ');
    assert_eq!(shabd.akshars[0].matra, varn.get_matra());
}

#[test]
fn test_shabd_process_chihn() {
    let mut shabd = Shabd::new();
    let swar_varn = Varn::from_char('आ'); // Swar Varn
    let chihn_varn = Varn::from_char('ऽ'); // Chihn Varn (Avgrah)

    // Populate the base VarnList with a Swar Varn
    shabd.base.push(swar_varn);

    shabd.process_chihn(chihn_varn);

    // One Akshar should have been created with both Swar and Avgrah Chihn
    assert_eq!(shabd.akshars.len(), 1);
    assert_eq!(shabd.akshars[0].akshar.varns.len(), 2);
    assert_eq!(shabd.akshars[0].akshar.varns[0].get_symbol(), 'आ');
    assert_eq!(shabd.akshars[0].akshar.varns[1].get_symbol(), 'ऽ');
}

#[test]
fn test_shabd_process_halant() {
    let mut shabd = Shabd::new();
    let vyanjan_varn1 = Varn::from_char('स'); // Vyanjan Varn
    let vyanjan_varn2 = Varn::from_char('त'); // Vyanjan Varn
    let halant_varn = Varn::from_char('्'); // Halant Varn
    let vyanjan_varn3 = Varn::from_char('य'); // Vyanjan Varn

    // Push Vyanjan onto the base VarnList
    shabd.base.push(vyanjan_varn1);
    shabd.base.push(vyanjan_varn2);
    shabd.base.push(halant_varn);
    shabd.base.push(vyanjan_varn3);

    shabd.make_akshars();

    // Two Akshars should be created सत् , य
    assert_eq!(shabd.akshars.len(), 2);
    assert_eq!(shabd.akshars[0].akshar.varns.len(), 3);
    assert_eq!(shabd.akshars[0].akshar.varns[0].get_symbol(), 'स');
    assert_eq!(shabd.akshars[0].akshar.varns[1].get_symbol(), 'त');
    assert_eq!(shabd.akshars[0].akshar.varns[2].get_symbol(), '्');
    assert_eq!(shabd.akshars[1].akshar.varns[0].get_symbol(), 'य');
}

#[test]
fn test_shabd_make_akshars() {
    let mut shabd = Shabd::new();
    shabd.base.push(Varn::from_char('क')); // Vyanjan
    shabd.base.push(Varn::from_char('्')); // Halant
    shabd.base.push(Varn::from_char('क')); // Vyanjan again
    shabd.base.push(Varn::from_char('ा')); // Chihn (Matra)

    shabd.make_akshars();

    // We should have 2 Akshars formed
    assert_eq!(shabd.akshars.len(), 2);

    // First Akshar should contain 'क' and '्'
    assert_eq!(shabd.akshars[0].akshar.varns.len(), 2);
    assert_eq!(shabd.akshars[0].akshar.varns[0].get_symbol(), 'क');
    assert_eq!(shabd.akshars[0].akshar.varns[1].get_symbol(), '्');

    // Second Akshar should contain 'क' and 'ा'
    assert_eq!(shabd.akshars[1].akshar.varns.len(), 2);
    assert_eq!(shabd.akshars[1].akshar.varns[0].get_symbol(), 'क');
    assert_eq!(shabd.akshars[1].akshar.varns[1].get_symbol(), 'ा');
}

#[test]
fn test_shabd_get_matra() {
    let mut shabd = Shabd::new();

    // Create Akshar with matra
    let mut akshar1 = Akshar::new();
    akshar1.matra = 2;

    let mut akshar2 = Akshar::new();
    akshar2.matra = 1;

    shabd.akshars.push(akshar1);
    shabd.akshars.push(akshar2);

    shabd.get_matra();

    assert_eq!(shabd.matra, 3); // Total matra should be the sum of all Akshars
}

#[test]
fn test_shabd_get_akshar_count() {
    let input = "श्याम";
    let shabd = Shabd::from_str(input);

    assert_eq!(shabd.akshar_count, 2);
}

#[test]
fn test_shabd_from_str() {
    let input = "रमण";
    let shabd = Shabd::from_str(input);

    assert_eq!(shabd.base.varns.len(), 3); // Checking if VarnList was created correctly
    assert_eq!(shabd.base.varns[0].get_symbol(), 'र');
    assert_eq!(shabd.base.varns[1].get_symbol(), 'म');
    assert_eq!(shabd.base.varns[2].get_symbol(), 'ण');

    // Test akshar processing
    assert_eq!(shabd.akshars.len(), 3);
    assert_eq!(shabd.akshars[0].akshar.varns[0].get_symbol(), 'र');
    assert_eq!(shabd.akshars[1].akshar.varns[0].get_symbol(), 'म');
    assert_eq!(shabd.akshars[2].akshar.varns[0].get_symbol(), 'ण');
}

#[test]
fn test_shabd_matra() {

    let mut input = "नमन";
    let mut shabd = Shabd::from_str(input);
    assert_eq!(shabd.akshars[0].matra, 1);
    assert_eq!(shabd.akshars[1].matra, 1);
    assert_eq!(shabd.akshars[2].matra, 1);

    input = "तुलसी";
    shabd = Shabd::from_str(input);
    assert_eq!(shabd.akshars[0].matra, 1);
    assert_eq!(shabd.akshars[1].matra, 1);
    assert_eq!(shabd.akshars[2].matra, 2);

    input = "हरीश";
    shabd = Shabd::from_str(input);
    assert_eq!(shabd.akshars[0].matra, 1);
    assert_eq!(shabd.akshars[1].matra, 2);
    assert_eq!(shabd.akshars[2].matra, 1);

    input = "कुमारी";
    shabd = Shabd::from_str(input);
    assert_eq!(shabd.akshars[0].matra, 1);
    assert_eq!(shabd.akshars[1].matra, 2);
    assert_eq!(shabd.akshars[2].matra, 2);

    input = "सावन";
    shabd = Shabd::from_str(input);
    assert_eq!(shabd.akshars[0].matra, 2);
    assert_eq!(shabd.akshars[1].matra, 1);
    assert_eq!(shabd.akshars[2].matra, 1);

    input = "मानुषी";
    shabd = Shabd::from_str(input);
    assert_eq!(shabd.akshars[0].matra, 2);
    assert_eq!(shabd.akshars[1].matra, 1);
    assert_eq!(shabd.akshars[2].matra, 2);

    input = "राधेय";
    shabd = Shabd::from_str(input);
    assert_eq!(shabd.akshars[0].matra, 2);
    assert_eq!(shabd.akshars[1].matra, 2);
    assert_eq!(shabd.akshars[2].matra, 1);

    input = "सावित्री";
    shabd = Shabd::from_str(input);
    assert_eq!(shabd.akshars[0].matra, 2);
    assert_eq!(shabd.akshars[1].matra, 2);
    assert_eq!(shabd.akshars[2].matra, 2);

    input = "पंकज";
    shabd = Shabd::from_str(input);
    assert_eq!(shabd.akshars[0].matra, 2);
    assert_eq!(shabd.akshars[1].matra, 1);
    assert_eq!(shabd.akshars[2].matra, 1);

    input = "पांडव";
    shabd = Shabd::from_str(input);
    assert_eq!(shabd.akshars[0].matra, 2);
    assert_eq!(shabd.akshars[1].matra, 1);
    assert_eq!(shabd.akshars[2].matra, 1);

    input = "बिंदु";
    shabd = Shabd::from_str(input);
    assert_eq!(shabd.akshars[0].matra, 2);
    assert_eq!(shabd.akshars[1].matra, 1);
}

#[test]
fn test_shabd_with_chihn() {
    let mut input = "हँस";
    let mut shabd = Shabd::from_str(input);

    assert_eq!(shabd.akshars.len(), 2);
    assert_eq!(shabd.matra, 2);
    assert_eq!(shabd.akshars[0].matra, 1);

    input = "गाँव";
    shabd = Shabd::from_str(input);

    assert_eq!(shabd.akshars.len(), 2);
    assert_eq!(shabd.matra, 3);
    assert_eq!(shabd.akshars[0].matra, 2);

    input  = "पढ़";
    shabd = Shabd::from_str(input);

    assert_eq!(shabd.akshars.len(), 2);
    assert_eq!(shabd.matra, 2);
    assert_eq!(shabd.akshars[0].matra, 1);
    assert_eq!(shabd.akshars[1].matra, 1);
}
