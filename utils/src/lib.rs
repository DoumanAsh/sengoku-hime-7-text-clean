#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate windows_win;

const GAME_CLASS: &'static str = "UnityWndClass";
const GAME_TITLE: &'static str = "戦極姫７遊戯強化版・弐";

#[macro_export]
macro_rules! error_println {
    ($($tt:tt)*) => {{
        use ::std::io::Write;
        let _ = writeln!(&mut ::std::io::stderr(), $($tt)*);
    }}
}

pub fn is_game_running() -> bool {
    windows_win::raw::window::find(GAME_CLASS, Some(GAME_TITLE)).is_ok()
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

pub fn extract_dialogue(text: &str) -> Option<String> {
    const BEGIN: &'static[char] = &['「', '（'];
    const END: &'static[char] = &['」', '）'];
    if let (Some(begin_pos), Some(end_pos)) = (text.find(BEGIN), text.rfind(END)) {
        let end_pos = end_pos + 3; //+3 to go at the symbol of dialogue end
        if end_pos == text.len() { return None; }

        Some(text[begin_pos..end_pos].to_string())
    }
    else {
        None
    }
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

fn remove_color(text: &str) -> Option<String> {
    lazy_static! {
        static ref RE_TAG: regex::Regex = regex::Regex::new("<[^>]+>").unwrap();
    }

    let result = RE_TAG.replace_all(text, "");

    if result.len() != text.len() {
        Some(result.to_string())
    }
    else {
        None
    }
}

///Processes text and returns changed text or None.
pub fn process_text(text: String) -> Option<String>{
    if is_jp(&text) {
        let text = text.trim();
        let text = extract_dialogue(text).unwrap_or(text.to_string());
        remove_color(&text).map(|text| remove_text_reps(text).replace("\n", ""))
    }
    else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{
        extract_dialogue,
        remove_text_reps,
        process_text
    };

    #[test]
    fn extract_dialogue_complex() {
        let text = "「ゲームは大きく分けて、「更新」「軍備」「内政」「政略」「作戦」「合戦」６つのフェ<color=#ffffff42>イ</color>「ゲームは大きく分けて、「更新」「軍備」「内政」「政略」「作戦」「合戦」６つのフェイズに分<color=#ffffff68>か</color>「ゲームは大きく分けて、「更新」「軍備」「内政」「政略」「作戦」「合戦」６つのフェイズに分かれてい<color=#ffffff8e>ま</color>「ゲームは大きく分けて、「更新」「軍備」「内政」「政略」「作戦」「合戦」６つのフェイズに分かれています<color=#ffffffff>」</color>";

        let expected_result = &text[..text.len() - 8];

        let result = extract_dialogue(text).expect("Extract dialogue");
        assert_eq!(result, expected_result);

        let expected_result = "「ゲームは大きく分けて、「更新」「軍備」「内政」「政略」「作戦」「合戦」６つのフェイズに分かれています」";
        let result = process_text(text.to_string()).expect("To process text");
        assert_eq!(result, expected_result);
    }

    #[test]
    fn extract_text() {
        let text = "この麗しき御方こそが、甲斐この麗しき御方こそが、甲斐源氏の本この麗しき御方こそが、甲斐源氏の本流たる武この麗しき御方こそが、甲斐源氏の本流たる武田家の第この麗しき御方こそが、甲斐源氏の本流たる武田家の第十九代目この麗しき御方こそが、甲斐源氏の本流たる武田家の第十九代目の当主。武この麗しき御方こそが、甲斐源氏の本流たる武田家の第十九代目の当主。武田信玄そこの麗しき御方こそが、甲斐源氏の本流たる武田家の第十九代目の当主。武田信玄その人だ。この麗しき御方こそが、甲斐源氏の本流たる武田家の第十九代目の当主。武田信玄その人だ。".to_string();

        let expected_result = "この麗しき御方こそが、甲斐源氏の本流たる武田家の第十九代目の当主。武田信玄その人だ。";

        let result = remove_text_reps(text);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn extract_text2() {
        let text = "手元に広げられた紙面に、ゆるり手元に広げられた紙面に、ゆるりと視線を手元に広げられた紙面に、ゆるりと視線を這わせる一手元に広げられた紙面に、ゆるりと視線を這わせる一人の 佳人手元に広げられた紙面に、ゆるりと視線を這わせる一人の 佳人。蝋燭の手元に広げられた紙面に、ゆるりと視線を這わせる一人の 佳人。蝋燭の淡い光に手元に広げられた紙面に、ゆるりと視線を這わせる一人の 佳人。蝋燭の淡い光に照らされ手元に広げられた紙面に、ゆるりと視線を這わせる一人の 佳人。蝋燭の淡い光に照らされるその横手元に広げられた紙面に、ゆるりと視線を這わせる一人の 佳人。蝋燭の淡い光に照らされるその横顔を、俺手元に広げられた紙面に、ゆるりと視線を這わせる一人の 佳人。蝋燭の淡い光に照らされるその横顔を、俺は無言で 見手元に広げられた紙面に、ゆるりと視線を這わせる一人の 佳人。蝋燭の淡い光に照らされるその横顔を、俺は無言で 見守り続け手元に広げられた紙面に、ゆるりと視線を這わせる一人の 佳人。蝋燭の淡い光に照らされるその横顔を、俺は無言で 見守り続ける。";

        let expected_result = "手元に広げられた紙面に、ゆるりと視線を這わせる一人の 佳人。蝋燭の淡い光に照らされるその横顔を、俺は無言で 見守り続ける。";

        let result = remove_text_reps(text.to_string());
        assert_eq!(result, expected_result);
    }

    #[test]
    fn extract_text3() {
        let text = "御館様の想定通り、信濃勢は御館様の想定通り、信濃勢は徹底抗戦の御館様の想定通り、信濃勢は徹底抗戦の構えを見御館様の想定通り、信濃勢は徹底抗戦の構えを見せた。";

        let expected_result = "御館様の想定通り、信濃勢は徹底抗戦の構えを見せた。";
        let result = remove_text_reps(text.to_string());
        assert_eq!(result, expected_result);
    }
}
