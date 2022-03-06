use super::Chars;

pub fn parse(mut chars: Chars<'_>) -> Option<(isize, isize, Chars<'_>)> {
	let parse_int = |chars: &mut Chars<'_>, breaker: char| {
		let mut num: String = chars.next().filter(|c| c.is_numeric() || *c == '-')?.to_string();
		loop {
			let c = chars.next()?;
			if c.is_numeric() {
				num.push(c)
			} else if c == breaker {
				break Some(num);
			} else {
				return None;
			}
		}
	};
	let num_a = parse_int(&mut chars, '.')?;
	if !matches!(chars.next(), Some('.')) {
		return None;
	}
	let num_b: String = parse_int(&mut chars, '}')?;
	Some((num_a.parse().ok()?, num_b.parse().ok()?, chars))
}
