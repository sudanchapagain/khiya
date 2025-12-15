use khiya_proc_macro::nepali;

nepali! {
    sar prakriya udaran() {
        paribhasa x = 5;
        paribhasa y = 3;
        paribhasa uttar = x * y;
        chhap!("{} guna {} = {}", x, y, uttar);
    }

    सार्वजनिक प्रक्रिया दोहोर्याउ() {
        परिभाषा परिवर्तनशील जम्मा = 0;
        परिभाषा परिवर्तनशील i = 1;
        जबसम्म i <= 5 {
            जम्मा += i;
            i += 1;
        }
        छाप!("जम्मा: {}", जम्मा);
    }

    sarbajanik prakriya udaran_dui(sankhya: i32) -> i32 {
        sankhya * sankhya
    }
}

fn main() {
    udaran();
    दोहोर्याउ();

    let sankhya = 4;
    println!("{}", udaran_dui(sankhya));
}
