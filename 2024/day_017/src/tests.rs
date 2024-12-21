use crate::Ucontrol;

#[test]
fn test_aoc() {
    assert_eq!(
        Ucontrol {
            reg_a: 0,
            reg_b: 0,
            reg_c: 9,
            i_p: 0,
            prog: String::from("26"),
            output: String::from("")
        }
        .eval()
        .reg_b,
        1
    );

    assert_eq!(
        Ucontrol {
            reg_a: 10,
            reg_b: 0,
            reg_c: 0,
            i_p: 0,
            prog: String::from("505154"),
            output: String::from("")
        }
        .eval()
        .output,
        String::from("0,1,2")
    );

    let mut uc = Ucontrol {
        reg_a: 2024,
        reg_b: 0,
        reg_c: 0,
        i_p: 0,
        prog: String::from("015430"),
        output: String::from(""),
    };
    uc.eval();
    assert_eq!(uc.output, String::from("4,2,5,6,7,7,7,7,3,1,0"));
    assert_eq!(uc.reg_a, 0);

    let mut uc = Ucontrol {
        reg_a: 0,
        reg_b: 29,
        reg_c: 0,
        i_p: 0,
        prog: String::from("17"),
        output: String::from(""),
    };
    uc.eval();
    assert_eq!(uc.reg_b, 26);

    let mut uc = Ucontrol {
        reg_a: 0,
        reg_b: 2024,
        reg_c: 43690,
        i_p: 0,
        prog: String::from("40"),
        output: String::from(""),
    };
    uc.eval();
    assert_eq!(uc.reg_b, 44354);

    let mut uc = Ucontrol {
        reg_a: 729,
        reg_b: 0,
        reg_c: 0,
        i_p: 0,
        prog: String::from("015430"),
        output: String::from(""),
    };
    uc.eval();
    assert_eq!(uc.output, "4,6,3,5,6,3,5,2,1,0");

    let mut uc = Ucontrol {
        reg_a: 55593699,
        reg_b: 0,
        reg_c: 0,
        i_p: 0,
        prog: String::from("2413750315445530"),
        output: String::from(""),
    };
    uc.eval();
    assert_ne!(uc.output, "3,2,3,0,2,4,1,5,3");

    let mut uc = Ucontrol {
        reg_a: 2024,
        reg_b: 0,
        reg_c: 0,
        i_p: 0,
        prog: String::from("035430"),
        output: String::from(""),
    };
    uc.eval();
    assert_ne!(uc.output, "0,3,5,4,3,0");
  
}
