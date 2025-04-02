use std::ffi::CString;
use std::io;


fn main() {
    println!("Please fill in the values!");
    println!("=============================");

    // declaring & getting the values
    let ak = parse_value("Aktien Kapital (AK)");
    let ges_gewinnres = parse_value("Ges Gewinnres.");
    let gewinnvortrag = parse_value("Gewinnvortrag");
    let jahresgewinn = parse_value("Jahresgewinn.");

    // math operations
    let bilanzgewinn:f32 = gewinnvortrag + jahresgewinn;
    let zuw_ges_gewinres:f32 = jahresgewinn / 20.00;
    let restlicher_bilanzgewinn:f32 = bilanzgewinn - zuw_ges_gewinres;
    let zuw_dividende_percentage:f32 = (restlicher_bilanzgewinn / ak * 100.00).floor();
    let zuw_dividende:f32 = ak / 100.00 * zuw_dividende_percentage;
    let neuer_gewinnvortrag:f32 = restlicher_bilanzgewinn - zuw_dividende;

    // add line spacing
    linewrap(5);

    // output the values in a table
    output_table(gewinnvortrag, jahresgewinn, bilanzgewinn, zuw_ges_gewinres, restlicher_bilanzgewinn, zuw_dividende, neuer_gewinnvortrag);

    // output buchungen
    output_buchungen(jahresgewinn, zuw_ges_gewinres);
}

fn parse_value(prompt: &str) -> f32 {
    let mut value = String::new();
    println!("{}", prompt);
    io::stdin()
        .read_line(&mut value)
        .expect("Failed to read line");

    loop {
        match value.trim().parse::<f32>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Invalid value, please provide an actual number for {} : ", prompt);
                value.clear();
                io::stdin()
                    .read_line(&mut value)
                    .expect("Failed to read line");
            }
        }
    }
}

fn output_table(
    gewinnvortrag: f32,
    jahresgewinn: f32,
    bilanzgewinn: f32,
    zuw_ges_gewinres: f32,
    restlicher_bilanzgewinn: f32,
    zuw_dividende: f32,
    neuer_gewinnvortrag: f32,
){
    split("Gewinnverteilungsplan");

    println!("----------------------------------+------------");
    println!("  Gewinnvortrag                   | {}", gewinnvortrag );
    println!("+ Jahresgewinn                    | {}", jahresgewinn );
    println!("----------------------------------+------------");
    println!("= Bilanzgewinn                    | {}", bilanzgewinn );
    println!("- Zuweisung ges. Gewinnres.       | {}", zuw_ges_gewinres );
    println!("----------------------------------+------------");
    println!("= Restlicher Bilanzgewinn         | {}", restlicher_bilanzgewinn );
    println!("- Zuweisung Dividende             | {}", zuw_dividende );
    println!("----------------------------------+------------");
    println!("= neuer Gewinnvortrag             | {}", neuer_gewinnvortrag);
}

fn output_buchungen(
    jahresgewinn: f32,
    zuw_ges_gewinres: f32,
) {
    split("Buchungen");

    println!("---------------+----------------------+------------");
    println!(" Gewinnvortrag | Ges. Gewinnreserve   | {}", zuw_ges_gewinres );
    println!("---------------+----------------------+-------------");
    println!(" Gewinnvortrag | Gewinnausschüttungen | {}", jahresgewinn );
    println!("---------------+----------------------+-------------");
}

fn split(prompt: &str) {
    println!("");
    println!("{}", prompt);
    println!("");
}

fn linewrap(value:u32){
    let mut count:u32 = 0;
    while count < value {
        println!();
        count+= 1;
    }
}