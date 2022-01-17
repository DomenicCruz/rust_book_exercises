fn main() {
    let start_cp = "On the first day of Christmas my true love sent to me";
    let gifts = [
    "a partridge in a pear tree",
    "two turtle doves",
    "three French hens",
    "four calling birds",
    "five gold rings",
    "six geese a-laying",
    "seven swans a-swimming",
    "eight maids a-milking",
    "nine ladies dancing",
    "ten lords a-leaping",
    "eleven pipers piping",
    "twelve drummers drumming"
    ];
   
    let mut count = 1;
    for gift in gifts {
        println!("{}", start_cp);
        for idx in (1..count).rev() {
            println!("{}", gifts[idx]);
        }
        if count > 1 {
            println!("and {}", gifts[0]);
        } else {
            println!("{}", gifts[0]);
        }
        println!("");
        count += 1;
    }
}
