use crate::property as prop;

pub fn add_fallbacks(props: &mut crate::Properties) {
	use crate::property::Property;
	match props.property::<prop::IndentSize>() {
		Some(Ok(prop::IndentSize::Value(n))) => {
			if let Some(Ok(prop::IndentStyle::Tabs)) = props.property::<prop::IndentStyle>() {
				let _ = props.try_insert(prop::TabWidth::key(), n.to_string());
			}
		}
		Some(Ok(prop::IndentSize::UseTabWidth)) => {
			if let Some(value) = props.get(prop::TabWidth::key()) {
				let value_owned = value.to_owned();
				std::mem::drop(value);
				let _ = props.insert(prop::IndentSize::key(), value_owned);
			}
		}
		_ => ()
	}
}
