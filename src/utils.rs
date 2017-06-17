#[macro_export]
macro_rules! error_println {
    ($($tt:tt)*) => {{
        use ::std::io::Write;
        let _ = writeln!(&mut ::std::io::stderr(), $($tt)*);
    }}
}

#[inline(always)]
pub fn is_jp<T: AsRef<str>>(text: T) -> bool {
    let text = text.as_ref();
    text.chars().any(|elem_char| match elem_char { '\u{3000}'...'\u{303f}'| //punctuation
                                                   '\u{3040}'...'\u{309f}'| //hiragana
                                                   '\u{30a0}'...'\u{30ff}'| //katakana
                                                   '\u{ff00}'...'\u{ffef}'| //roman characters
                                                   '\u{4e00}'...'\u{9faf}'| //common kanji
                                                   '\u{3400}'...'\u{4dbf}'  //rare kanji
                                                      => true,
                                                   _  => false,
    })
}

pub fn remove_text_reps(text: String) -> String {
    let chars = text.chars().collect::<Vec<_>>();
    let mut pred = Vec::new();

    for idx in 1..chars.len() {
        pred.push(chars[idx - 1]);

        if chars[idx..].starts_with(&pred) {
            for r_idx in (1..chars.len()).rev() {
                if chars[r_idx..].starts_with(&pred) {
                    return chars[r_idx..].iter().fold(String::new(), |mut acc, ch| { acc.push(*ch); acc});
                }
            }
        }
    }

    text
}

#[cfg(test)]
mod tests {
    #[test]
    fn extract_text() {
        use super::remove_text_reps;
        let text = "この麗しき御方こそが、甲斐この麗しき御方こそが、甲斐源氏の本この麗しき御方こそが、甲斐源氏の本流たる武この麗しき御方こそが、甲斐源氏の本流たる武田家の第この麗しき御方こそが、甲斐源氏の本流たる武田家の第十九代目この麗しき御方こそが、甲斐源氏の本流たる武田家の第十九代目の当主。武この麗しき御方こそが、甲斐源氏の本流たる武田家の第十九代目の当主。武田信玄そこの麗しき御方こそが、甲斐源氏の本流たる武田家の第十九代目の当主。武田信玄その人だ。この麗しき御方こそが、甲斐源氏の本流たる武田家の第十九代目の当主。武田信玄その人だ。".to_string();

        let expected_result = "この麗しき御方こそが、甲斐源氏の本流たる武田家の第十九代目の当主。武田信玄その人だ。";

        let result = remove_text_reps(text);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn extract_text2() {
        use super::remove_text_reps;
        let text = "手元に広げられた紙面に、ゆるり手元に広げられた紙面に、ゆるりと視線を手元に広げられた紙面に、ゆるりと視線を這わせる一手元に広げられた紙面に、ゆるりと視線を這わせる一人の 佳人手元に広げられた紙面に、ゆるりと視線を這わせる一人の 佳人。蝋燭の手元に広げられた紙面に、ゆるりと視線を這わせる一人の 佳人。蝋燭の淡い光に手元に広げられた紙面に、ゆるりと視線を這わせる一人の 佳人。蝋燭の淡い光に照らされ手元に広げられた紙面に、ゆるりと視線を這わせる一人の 佳人。蝋燭の淡い光に照らされるその横手元に広げられた紙面に、ゆるりと視線を這わせる一人の 佳人。蝋燭の淡い光に照らされるその横顔を、俺手元に広げられた紙面に、ゆるりと視線を這わせる一人の 佳人。蝋燭の淡い光に照らされるその横顔を、俺は無言で 見手元に広げられた紙面に、ゆるりと視線を這わせる一人の 佳人。蝋燭の淡い光に照らされるその横顔を、俺は無言で 見守り続け手元に広げられた紙面に、ゆるりと視線を這わせる一人の 佳人。蝋燭の淡い光に照らされるその横顔を、俺は無言で 見守り続ける。";

        let expected_result = "手元に広げられた紙面に、ゆるりと視線を這わせる一人の 佳人。蝋燭の淡い光に照らされるその横顔を、俺は無言で 見守り続ける。";

        let result = remove_text_reps(text.to_string());
        assert_eq!(result, expected_result);
    }

    #[test]
    fn extract_text3() {
        use super::remove_text_reps;
        let text = "御館様の想定通り、信濃勢は御館様の想定通り、信濃勢は徹底抗戦の御館様の想定通り、信濃勢は徹底抗戦の構えを見御館様の想定通り、信濃勢は徹底抗戦の構えを見せた。";

        let expected_result = "御館様の想定通り、信濃勢は徹底抗戦の構えを見せた。";
        let result = remove_text_reps(text.to_string());
        assert_eq!(result, expected_result);
    }
}
