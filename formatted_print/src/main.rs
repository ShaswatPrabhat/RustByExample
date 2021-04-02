fn main() {
    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    println!("{} days", 31);
    println!("{0} this is {1}. {1}, this is {0}", "Bubu", "Zuzu");
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);
}
