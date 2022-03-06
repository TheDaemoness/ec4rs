use crate::property as prop;

pub fn add_fallbacks(props: &mut crate::Properties) {
	use crate::property::Property;
	match props.get::<prop::IndentSize>() {
		Ok(prop::IndentSize::Value(n)) => {
			if let Ok(prop::IndentStyle::Tabs) = props.get::<prop::IndentStyle>() {
				let _ = props.try_insert_raw_for_key(prop::TabWidth::key(), n.to_string());
			}
		}
		Ok(prop::IndentSize::UseTabWidth) => {
			if let Some(value) = props.get_raw::<prop::TabWidth>().value() {
				let value_owned = value.to_owned();
				let _ = props.insert_raw_for_key(prop::IndentSize::key(), value_owned);
			}
		}
		_ => ()
	}
}
