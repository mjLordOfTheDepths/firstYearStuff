

fn main() {

    println!("Haiiii :3 So if you want to convert Toki Pona Nanpa to English enter '1', else enter any other character!");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().to_uppercase();
    
    if input == "1" {
        println!("Enter a numeric value in Toki Pona >_< :");

        let mut total: i32 = 0;
        let mut okSoIAccidentallyUsedTotalForTheTemporaryVariableSoNowThisWillBeTheRealTotalIPromise: i32= 0;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let inputButAVector: Vec<&str> = input.split_whitespace().collect();

        for input in inputButAVector {
            match input.to_uppercase().as_str() {
                "MUTE" => total = 20,
                "WAN" => total = 1,
                "TU" => total = 2,
                "ALE" => total = 100,
                "LUKA" => total = 5,
                _ => total = 0,

            }

            okSoIAccidentallyUsedTotalForTheTemporaryVariableSoNowThisWillBeTheRealTotalIPromise += total;

        }

        println!("Yuor total is around {}", okSoIAccidentallyUsedTotalForTheTemporaryVariableSoNowThisWillBeTheRealTotalIPromise);
    }

    else {

        let mut céadta: i32 = 0;
        let mut scóir: i32 = 0;
        let mut cúigí: i32 = 0;
        let mut triúr: i32 = 0;
        let mut aon: i32 = 0;
        let mut n: i32 = 0;
        let mut i: i32 = 0;

        // NO!!!!! THREE DOESN'T EVEN EXIST IN THE LANGUAGE YUOR CODE IS REDUNDANT!!!! WHAT DO YOU MEAN YOU'RE GOING TO USE 3 FOR 2 ???

        println!("Enter yuor favourite positive integer and the little gnome inside your computer will do his best to translate it for you: ");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut input: i32 = input.trim().parse().unwrap();

        while (input > 0) {
            céadta = (input - 100).abs();
            scóir = (input - 20).abs();
            cúigí = (input - 5).abs();
            triúr = (input - 2).abs();
            aon = (input - 1).abs();

            if céadta <= scóir {
                if input - 100 >= 0 {
                    print!("Ale ");
                    input -= 100;
                }
                
                else if input - 100 < 0 && input != 0 {
                    print!("Ale Weka ");
                    input = (input - 100).abs();
                }
            }

            else if scóir <= cúigí {
                if input - 20 >= 0 {
                    print!("Mute ");
                    input -= 20;
                }

                else if input - 20 < 0 && input != 0 {
                    print!("Mute Weka ");
                    input = (input - 20).abs();
                }
            }

            else if cúigí <= triúr {
                if input - 5 >= 0 {
                    print!("Luka ");
                    input -= 5;
                }

                else if input - 5 < 0 && input != 0 {
                    print!("Luka Weka ");
                    input = (input - 5).abs();
                }
            }

            else if triúr <= aon {
                if input - 2 >= 0 {
                    print!("Tu ");
                    input -= 2;
                }

                else if input - 2 < 0 && input != 0 {
                    print!("Tu Weka ");
                    input = (input - 2).abs();
                }
            }

            else {
                if input - 1 >= 0 {
                    print!("Wan ");
                    input -= 1;
                }

                else if input - 1 < 0 && input != 0 {
                    print!("Wan Weka ");
                    input = (input - 1).abs();
                }
            }
        }


    }

    println!("")
}
