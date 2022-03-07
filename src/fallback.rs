use crate::property as prop;

pub fn add_fallbacks(props: &mut crate::Properties, legacy: bool) {
	let val = props.get_raw::<prop::IndentSize>();
	if let Some(value) = val.value() {
		if let Ok(prop::IndentSize::UseTabWidth) = val.parse::<prop::IndentSize, false>() {
			let value = props.get_raw::<prop::TabWidth>().value().unwrap_or("tab").to_owned();
			props.insert_raw::<prop::IndentSize, _>(value);
		} else {
			let value = value.to_owned();
			let _ = props.try_insert_raw::<prop::TabWidth, _>(value);
		}
	} else {
		let value = props
			.get_raw::<prop::TabWidth>()
			.filter_unset()
			.into_result()
			.unwrap_or("")
			.to_owned();
		let _ = props.try_insert_raw::<prop::IndentSize, _>(value);
	}
	if !legacy {
		if let Ok(prop::IndentStyle::Tabs) = props.get::<prop::IndentStyle>() {
			let _ = props.try_insert(prop::IndentSize::UseTabWidth);
		}
	}
}
