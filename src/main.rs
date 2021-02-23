#![feature(assoc_char_funcs)]
fn main() {
    print!("{}", diamond('G'));
}

fn generate_line(line_type: char) -> String {
    if line_type != 'A' {
        let gap_size = 2 * (line_type as u32 - 'A' as u32) as usize - 1;
        return format!("{0}{1}{0}", line_type.to_string(), &" ".repeat(gap_size));
    }

    line_type.to_string()
}

fn diamond(diamond_type: char) -> String {
    const BASE_LETTER: u32 = 'A' as u32;
    let diamond_height: usize = (diamond_type as u32 - BASE_LETTER) as usize;

    let mut result: String = "".to_string();

    for line_number in 0..diamond_height + 1 {
        result.push_str(&add_line(diamond_height, line_number, BASE_LETTER));
    }

    for line_number in 1..diamond_height + 1 {
        let line_number = diamond_height - line_number;
        result.push_str(&add_line(diamond_height, line_number, BASE_LETTER));
    }

    result
}

fn add_line(height: usize, line_number: usize, base: u32) -> String {
    let mut result = "".to_string();
    let buffer: &str = &" ".repeat(height - line_number);
    let letter: u32 = line_number as u32 + base;

    result.push_str(buffer);
    result.push_str(generate_line(char::from_u32(letter).unwrap()).as_str());
    result + "\n"
}

mod test {
    use super::*;

    #[test]
    fn first_line_of_diamond_a() {
        assert_eq!(diamond('A').split("\n").collect::<Vec<&str>>()[0], "A");
    }

    #[test]
    fn first_line_of_diamond_b() {
        assert_eq!(diamond('B').split("\n").collect::<Vec<&str>>()[0], " A");
    }

    #[test]
    fn first_line_of_diamond_c() {
        assert_eq!(diamond('C').split("\n").collect::<Vec<&str>>()[0], "  A");
    }

    #[test]
    fn second_line_of_diamond_a() {
        assert_eq!(diamond('A').split("\n").collect::<Vec<&str>>()[1], "");
    }

    #[test]
    fn second_line_of_diamond_b() {
        assert_eq!(diamond('B').split("\n").collect::<Vec<&str>>()[1], "B B");
    }

    #[test]
    fn second_line_of_diamond_c() {
        assert_eq!(diamond('C').split("\n").collect::<Vec<&str>>()[1], " B B");
    }

    #[test]
    fn third_line_of_diamond_c() {
        assert_eq!(diamond('C').split("\n").collect::<Vec<&str>>()[2], "C   C");
    }

    #[test]
    fn third_line_of_diamond_d() {
        assert_eq!(diamond('D').split("\n").collect::<Vec<&str>>()[2], " C   C");
    }

    #[test]
    fn fourth_line_diamond_d() {
        assert_eq!(
            diamond('D').split("\n").collect::<Vec<&str>>()[3],
            "D     D"
        );
    }

    #[test]
    fn generate_line_a() {
        assert_eq!(generate_line('A'), "A");
    }

    #[test]
    fn generate_line_b() {
        assert_eq!(generate_line('B'), "B B");
    }

    #[test]
    fn generate_line_c() {
        assert_eq!(generate_line('C'), "C   C");
    }

    #[test]
    fn third_line_of_diamond_b() {
        assert_eq!(diamond('B').split("\n").collect::<Vec<&str>>()[2], " A");
    }

    #[test]
    fn fourth_line_of_diamond_c() {
        assert_eq!(diamond('C').split("\n").collect::<Vec<&str>>()[3], " B B");
    }
}
