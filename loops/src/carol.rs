fn main() {
    let mut count = 0;
    let lyrics = ["partridge in a pear tree", "Two turtle doves,", "Three French hens,", "Four calling birds,", "Five gold rings,", "Six geeze a-laying,", "Seven swans a-swimming,", "Eight maids a-milking,", "Nine ladies dancing,", "Ten lords a-leaping,", "Eleven pipers piping,", "Twelve drummers drumming,"];

    while count != 12 {
        println!("\nOn the first day of Christmas my true love sent to me");
        let mut count1 = count;

        if count == 0 {
            println!("A {}.", lyrics[count]);
        } else {
            println!("{}", lyrics[count]);
            while count1 != 0 {
                if count1 == 1 {
                    println!("And a {}.", lyrics[count1 - 1])
                } else {
                    println!("{}", lyrics[count1 - 1]);
                }
                count1 -= 1;
            }
        }

        count += 1;
    }
}
