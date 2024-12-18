use std::iter;

pub const SYMBOL_HEIGHT: u16 = 5;
pub const SYMBOL_WIDHT: u16 = 6;

fn contcat_ascii_numbers(n1: &str, n2: &str) -> String {
    let mut concated = String::new();
    for (l1, l2) in iter::zip(n1.lines(), n2.lines()) {
        concated += format!("{:06}", l1).as_str();
        concated += " ";
        concated += format!("{:06}", l2).as_str();
        concated += "\n";
    }
    concated
}

pub fn number_as_asciiart(number: u64) -> String {
    let number_ascii = number.to_string();
    let mut ascii_numbers = number_ascii
        .chars()
        .map(char_as_asciiart)
        .collect::<Vec<&str>>();
    if ascii_numbers.len() < 2 {
        ascii_numbers.insert(0, char_as_asciiart('0'))
    }
    contcat_ascii_numbers(ascii_numbers[0], ascii_numbers[1])
}

pub fn char_as_asciiart(c: char) -> &'static str {
    match c {
        '0' => {
            "######
##  ##
##  ##
##  ##
######"
        }
        '1' => {
            "####
  ##
  ##
  ##
######"
        }
        '2' => {
            "######
     #
######
#
######"
        }
        '3' => {
            "######
    ##
######
    ##
######"
        }
        '4' => {
            "##  ##
##  ##
######
    ##
    ##"
        }
        '5' => {
            "######
##
######
    ##
######"
        }
        '6' => {
            "######
##
######
##  ##
######"
        }
        '7' => {
            "######
    ##
    ##
    ##
    ##"
        }
        '8' => {
            "######
##  ##
######
##  ##
######"
        }
        '9' => {
            "######
##  ##
######
    ##
######"
        }
        _ => unreachable!(),
    }
}
