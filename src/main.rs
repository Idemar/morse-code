#[derive(Debug, PartialEq)]
enum Puls {
    Kort,
    Lang,
}

type Bokstav = Vec<Puls>;

type Melding = Vec<Bokstav>;

trait MorseCode {
    fn til_morse_code(&self) -> Melding;
}
impl MorseCode for String {
    fn til_morse_code(&self) -> Melding {
        use Puls::*;
        let mut msg = Vec::with_capacity(self.len());

        for b in self.chars() {
            let morse = match b {
                'A' | 'a' => vec![Kort, Lang],
                'B' | 'b' => vec![Lang, Kort, Kort, Kort],
                'C' | 'c' => vec![Lang, Kort, Lang, Kort],
                'D' | 'd' => vec![Lang, Kort, Kort],
                'E' | 'e' => vec![Kort],
                'F' | 'f' => vec![Kort, Kort, Lang, Kort],
                'G' | 'g' => vec![Lang, Lang, Kort],
                'H' | 'h' => vec![Kort, Kort, Kort, Kort],
                'I' | 'i' => vec![Kort, Kort],
                'J' | 'j' => vec![Kort, Lang, Lang, Lang],
                'K' | 'k' => vec![Lang, Kort, Lang],
                'L' | 'l' => vec![Kort, Lang, Kort, Kort],
                'M' | 'm' => vec![Lang, Lang],
                'N' | 'n' => vec![Lang, Kort],
                'O' | 'o' => vec![Lang, Lang, Lang],
                'P' | 'p' => vec![Kort, Lang, Lang, Kort],
                'Q' | 'q' => vec![Lang, Lang, Kort, Lang],
                'R' | 'r' => vec![Kort, Lang, Kort],
                'S' | 's' => vec![Kort, Kort, Kort],
                'T' | 't' => vec![Lang],
                'U' | 'u' => vec![Kort, Kort, Lang],
                'V' | 'v' => vec![Kort, Kort, Kort, Lang],
                'W' | 'w' => vec![Kort, Lang, Lang],
                'X' | 'x' => vec![Lang, Kort, Kort, Lang],
                'Y' | 'y' => vec![Lang, Kort, Lang, Lang],
                'Z' | 'z' => vec![Lang, Lang, Kort, Kort],

                '1' => vec![Kort, Lang, Lang, Lang, Lang],
                '2' => vec![Kort, Kort, Lang, Lang, Lang],
                '3' => vec![Kort, Kort, Kort, Lang, Lang],
                '4' => vec![Kort, Kort, Kort, Kort, Lang],
                '5' => vec![Kort, Kort, Kort, Kort, Kort],
                '6' => vec![Lang, Kort, Kort, Kort, Kort],
                '7' => vec![Lang, Lang, Kort, Kort, Kort],
                '8' => vec![Lang, Lang, Lang, Kort, Kort],
                '9' => vec![Lang, Lang, Lang, Lang, Kort],
                '0' => vec![Lang, Lang, Lang, Lang, Lang],
                _ => continue,
            };
            msg.push(morse);
        }
        msg
    }
}

impl std::fmt::Display for Puls {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Puls::Kort => write!(f, "*"),
            Puls::Lang => write!(f, "-"),
        }
    }
}

fn print_morse_code(code: &Melding) {
    for bokstav in code.iter() {
        for plus in bokstav.iter() {
            print!("{}", plus);
        }
        print!(" ");
    }
    println!();
}

fn main() {
    let morse_melding = "Hello world".to_string().til_morse_code();

    print_morse_code(&morse_melding);
}

#[test]
fn hello_world() {
    use Puls::*;

    let forventet = vec![
        vec![Kort, Kort, Kort, Kort],
        vec![Kort],
        vec![Kort, Lang, Kort, Kort],
        vec![Kort, Lang, Kort, Kort],
        vec![Lang, Lang, Lang],
        vec![Kort, Lang, Lang],
        vec![Lang, Lang, Lang],
        vec![Kort, Lang, Kort],
        vec![Kort, Lang, Kort, Kort],
        vec![Lang, Kort, Kort],
    ];

    let source = "Hello world";
    let actual = source.to_string().til_morse_code();

    assert_eq!(actual, forventet);
}

#[test]
fn whole_alphabet() {
    let alphabet = "abcdefghijklmopqrstuvwxyz1234567890".to_string();

    let lower = alphabet.til_morse_code();
    let upper = alphabet.to_uppercase().til_morse_code();

    assert_eq!(lower, upper);
}
