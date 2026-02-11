use callfind::grid_locator::GridLocator;
use compact_str::CompactString;
use mlua::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Misc {
    pub antenna: Option<CompactString>,
    pub power: Option<f64>,
    pub operator: Option<CompactString>,
    pub address: Option<CompactString>,
    pub grid: Option<GridLocator>,
    pub manager: Option<CompactString>,
}

impl LuaUserData for Misc {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("antenna", |_, this| {
            Ok(this.antenna.as_ref().map(ToString::to_string))
        });
        fields.add_field_method_get("power", |_, this| Ok(this.power));
        fields.add_field_method_get("operator", |_, this| {
            Ok(this.operator.as_ref().map(ToString::to_string))
        });
        fields.add_field_method_get("address", |_, this| {
            Ok(this.address.as_ref().map(ToString::to_string))
        });
        fields.add_field_method_get("grid", |_, this| Ok(this.grid.map(|s| s.to_string())));
    }
}
