use std::{
    fmt::Write as _,
    io::{Write, stdin, stdout},
};

struct Spell {
    name: Box<str>,
    source: Box<str>,
    page: Box<str>,
    level: Box<str>,
    casting_time: Box<str>,
    duration: Box<str>,
    school: Box<str>,
    range: Box<str>,
    components: Box<str>,
    text: Box<str>,
    at_higher_levels: Box<str>,
}

enum Justification {
    LEFT = 0,
    CENTER = 1,
    RIGHT = 2,
}

fn select_justification(justification: Justification) {
    let _ = stdout().write(&[0x1B, 0x61, justification as u8]);
}

fn init() {
    let _ = stdout().write(&[0x1B, 0x40]);
}

fn select_character_size(width: u8, height: u8) {
    let size = height | (width << 4);
    let _ = stdout().write(&[0x1D, 0x21, size]);
}

fn sides(left: &str, right: &str) -> [u8; 42] {
    let mut line = [0x20; 42];
    line[..left.len()].copy_from_slice(left.as_bytes());
    line[42 - right.len()..].copy_from_slice(right.as_bytes());
    line
}

fn wrap(src: &str) -> Box<str> {
    let mut out = String::with_capacity(src.len());
    let mut x = 0;
    let mut word = String::new();
    for c in src.chars() {
        if c == ' ' {
            if x + word.len() >= 42 {
                let _ = out.write_char('\n');
                x = 0;
            } else {
                let _ = out.write_char(' ');
                x += 1;
            }
            let _ = out.write_str(&word);
            x += word.len();
            word.clear();
        } else if c == '\n' {
            if x + word.len() >= 42 {
                let _ = out.write_char('\n');
            } else {
                let _ = out.write_char(' ');
            }
            let _ = out.write_str(&word);
            let _ = out.write_char('\n');
            x = 0;
            word.clear();
        }
        else {
            let _ = word.write_char(c);
        }
    }
    Box::from(out)
}

fn feed_and_cut() {
    let _ = stdout().write(&[0x1D, 0x56, 65, 210]);
}

fn print_spell(spell: &Spell) {
    select_justification(Justification::CENTER);
    select_character_size(1, 1); //Double size
    print!("{}", spell.name);
    select_character_size(0, 0);
    print!("\n");
    select_justification(Justification::LEFT);
    let _ = stdout().write(&sides("Level", &spell.level));
    print!("\n");
    let _ = stdout().write(&sides("Casting time", &spell.casting_time));
    print!("\n");
    let _ = stdout().write(&sides("Range", &spell.range));
    print!("\n");
    let _ = stdout().write(&sides("Duration", &spell.duration));
    print!("\n");
    let _ = stdout().write(&sides("School", &spell.school));
    print!("\n");
    print!("\n");
    println!("{}", wrap(&spell.text));
    println!("{}", wrap(&spell.at_higher_levels));
    feed_and_cut();
}

fn process_spells() -> Option<()> {
    init();
    let mut reader = csv::Reader::from_reader(stdin());
    for result in reader.records() {
        let record = result.ok()?;
        let spell = Spell {
            name: Box::from(record.get(0)?),
            source: Box::from(record.get(1)?),
            page: Box::from(record.get(2)?),
            level: Box::from(record.get(3)?),
            casting_time: Box::from(record.get(4)?),
            duration: Box::from(record.get(5)?),
            school: Box::from(record.get(6)?),
            range: Box::from(record.get(7)?),
            components: Box::from(record.get(8)?),
            text: Box::from(record.get(12)?),
            at_higher_levels: Box::from(record.get(13)?),
        };
        print_spell(&spell);
    }
    Some(())
}

fn main() {
    process_spells();
}
