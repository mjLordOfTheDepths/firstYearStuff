use std::iter::Successors;

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

        // NO!!!!! YOU'RE MEANT TO JUST RE-USE THE SAME VARIABLE!!! YOU DON'T NEED A DIFFERENT VARIABLE FOR 1s, 2s, 5s, 20s, AND 100s!!! THREE DOESN'T EVEN EXIST IN THE LANGUAGE YUOR CODE IS REDUNDANT!!!! WHAT DO YOU MEAN YOU'RE GOING TO USE 3 FOR 2 ???

        println!("Enter yuor favourite positive integer and the little gnome inside your computer will do his best to translate it for you: ");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input: i32 = input.trim().parse().unwrap();

        if (input / 100) > 0 {
            céadta = input / 100;
            for _ in 0..céadta {print!("Ale ")}
        }

        if input % 100 > 50 {
            print!("Ale Weka ");
            let mut n = input % 100;
            i = n / 20;
            while i > 1 {i -= 1; print!("Mute "); if i > 1 {print!("Weka ")}}
            n = input % 20;
            match n {
                1 => print!("Wan "),
                2 => print!("Tu "),
                3 => print!("Wan Tu "),
                4 => print!("Tu Tu "),
                5 => print!("Luka "),
                6 => print!("Luka Wan "),
                7 => print!("Luka Tu "),
                8 => print!("Luka Tu Wan "),
                9 => print!("Luka Tu Tu "),
                10 => print!("Luka Luka "),
                11 => print!("Luka Luka Wan"),
                12 => print!("Luka Luka Tu "),
                13 => print!("Mute Weka Luka Meka Tu "),
                14 => print!("Mute Weka Luka Meka Wan"),
                15 => print!("Mute Weka Luka "),
                16 => print!("Mute Weka Tu Weka Tu "),
                17 => print!("Mute Weka Tu Weka Wan "),
                18 => print!("Mute Weka Tu "),
                19 => print!("Mute Weka Wan "),
                _ => print!(" "),
            }
        }



    }
}
