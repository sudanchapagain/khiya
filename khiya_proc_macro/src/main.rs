use khiya_proc_macro::nepali;

fn main() {
    nepali! {
        /*
        ALTERNATIVE
        ===========
        sarbajanik prakriya mukhya() {
            paribhasa x = 42;
            yedi x > 0 {
                chappankti!("nepali");
            }
        }
        */
        सार्वजनिक प्रक्रिया मुख्य() {
            परिभाषा x = 42;
            यदि x > 0 {
                छापपङ्क्ति!("nepali");
            }
        }
    }
}
