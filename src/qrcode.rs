use clap::ArgMatches;
use qrcode::QrCode;

pub fn qrcode_command(sub_matches: &ArgMatches) {
    if let Some(string) = sub_matches.get_one::<String>("STRING") {
        let code = QrCode::new(format!("{string}").as_bytes()).unwrap();
        let string = code
            .render::<char>()
            .quiet_zone(false)
            .module_dimensions(2, 1)
            .build();
        println!("{}", string);
    }
}
