/// print banners
/// # Tests
/// ```
/// use banner::print_banner;
/// print_banner();
///
/// assert_eq!(1+1, 2,"basic math is not correct");
/// ```
pub fn print_banner() {
    println!("     ___   ____    ____ ____    ____  __   __       _______   __   ________    .______          _______.
    /   \\  \\   \\  /   / \\   \\  /   / |  | |  |     |       \\ |  | |       /    |   _  \\        /       |
   /  ^  \\  \\   \\/   /   \\   \\/   /  |  | |  |     |  .--.  ||  | `---/  /     |  |_)  |      |   (----`
  /  /_\\  \\  \\_    _/     \\_    _/   |  | |  |     |  |  |  ||  |    /  /      |      /        \\   \\
 /  _____  \\   |  |         |  |     |  | |  `----.|  '--'  ||  |   /  /----.__|  |\\  \\----.----)   |
/__/     \\__\\  |__|         |__|     |__| |_______||_______/ |__|  /________(__) _| `._____|_______/
    ");
}
