fn main() {
    let pi = 3.141592;
    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    println!("{} days", 31);
    println!("{0} this is {1}. {1}, this is {0}", "Bubu", "Zuzu");
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );
    println!(
        "{} of {:b} people know binary, the other half doesn't",
        1, 2
    );
    // Attempt to right align printed output
    println!("{number:>0width$}", number = 420, width = 10);
    // Attempt to control number of decimal digits on pi
    println!("Value of {} is {:.3}", "pi", pi);
}
