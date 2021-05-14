use std::thread::sleep;
use std::time::Duration;

fn main() {
    // TODO: Draw a better version of a tire.
    let mut tire = "
            .-/+oossssoo+/-.
        `:+sssssssssssssssdMMN:`
      -+sssssssssssssssssdMMMNys+-
    .ossssssssssssssssssdMMMNysssso.
   /ssssssssssshdmmNNmmyNMMMMhssssss/
  +ssssssssshmydMMMMMMMNddddyssssssss+
 /sssssssshNMMMyhhyyyyhmNMMMNhssssssss/
.ssssssssdMMMNhsssssssssshNMMMdssssssss.
+sssshhhyNMMNyssssoooossssyNMMMysssssss+
yNNNNMMMNyMMhssssoOOOOosssshmmmhssssssso
yNNNNMMMNyMMhssssoOOOOosssshmmmhssssssso
+sssshhhyNMMNyssssoooossssyNMMMysssssss+
.ssssssssdMMMNhsssssssssshNMMMdssssssss.
 /sssssssshNMMMyhhyyyyhdNMMMNhssssssss/
  +sssssssssdmydMMMMMMMMddddyssssssss+
   /ssssssssssshdmNNNNmyNMMMMhssssss/
    .ossssssssssssssssssdMMMNysssso.
      -+sssssssssssssssssyMMMNys+-
        `:+ssssssssssssssssMMMN`
            .-/+oossssoo+/-.
";

    loop {
        // TODO: Modify lines of tire
        for mut line in tire.lines() {
            line = &(" ".to_owned() + line);
        }

        print!("\x1B[2J\x1B[1;1H"); // clear terminal screen
        println!("{}", tire);
        sleep(Duration::from_millis(1000));
    }
}
