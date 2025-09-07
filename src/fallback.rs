use crate::property as prop;

pub fn add_fallbacks(props: &mut crate::Properties, legacy: bool) {
    let val = props.get_raw::<prop::IndentSize>();
    if let Some(value) = val {
        if let Ok(prop::IndentSize::UseTabWidth) = value.parse::<prop::IndentSize>() {
            let value = props
                .get_raw::<prop::TabWidth>()
                .cloned()
                .unwrap_or(crate::string::SharedString::new_static("tab"));
            props.insert_raw::<prop::IndentSize, _>(value);
        } else {
            let value = value.to_owned();
            let _ = props.try_insert_raw::<prop::TabWidth, _>(value);
        }
    } else if let Some(value) = props
        .get_raw::<prop::TabWidth>()
        .filter(|v| *v != &crate::string::UNSET)
    {
        let _ = props.try_insert_raw::<prop::IndentSize, _>(value.to_owned());
    }
    if !legacy {
        if let Ok(prop::IndentStyle::Tabs) = props.get::<prop::IndentStyle>() {
            let _ = props.try_insert(prop::IndentSize::UseTabWidth);
        }
    }
}
