use crate::property as prop;

pub fn add_fallbacks(props: &mut crate::Properties) {
	use crate::property::Property;
	match props.property::<prop::IndentSize>() {
		Ok(prop::IndentSize::Value(n)) => {
			if let Ok(prop::IndentStyle::Tabs) = props.property::<prop::IndentStyle>() {
				let _ = props.try_insert(prop::TabWidth::key(), n.to_string());
			}
		}
		Ok(prop::IndentSize::UseTabWidth) => {
			if let Some(value) = props.get(prop::TabWidth::key()) {
				let value_owned = value.to_owned();
				let _ = props.insert(prop::IndentSize::key(), value_owned);
			}
		}
		_ => ()
	}
}
