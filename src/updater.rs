use std::process::Command;
use std::str::from_utf8;
const COMMAND: &'static str = r#"pactl list sink-inputs | awk -f"#;
const DEFAULT_FILE_NAME: &'static str = r#"~/.config/utils/pactl-parser.awk"#;

#[derive(Debug)]
pub struct Data {
    pub id:     usize,
    pub name:   String,
    pub volume: usize,
    pub mute:   bool,
}

pub fn update() -> Vec<Data> {

    let cmd = Command::new("bash")
        .args(&["-c", &format!("{} {}", COMMAND, DEFAULT_FILE_NAME)])
        .output()
        .expect("can't execute command");

    from_utf8(&cmd.stdout)
	.unwrap()
	.lines()
	.map(|v| { let tmp = v.split(" ").collect::<Vec<_>>();
		   Data { id:     tmp.get(0).unwrap().parse().unwrap(),
			  name:   tmp.get(1).unwrap().to_string(),
			  volume: tmp.get(2).unwrap().parse().unwrap(),
			  mute:   tmp.get(3).unwrap().to_owned() == "yes"}})
        .collect::<Vec<_>>()

}

pub fn call(id: usize, value: usize) {
    call_local(id, value);
}

// pub fn mute(id: usize, status: bool) {
//     Command::new("pactl")
//         .args(&["set-sink-input-mute", &(!status).to_string()])
//         .status()
//         .unwrap();
// }

fn call_local(id: usize, value: usize) {
    Command::new("pactl")
        .args(&["set-sink-input-volume", &id.to_string(), &value.to_string()])
        .status()
	.unwrap();
}

