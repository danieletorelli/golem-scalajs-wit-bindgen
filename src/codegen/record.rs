use std::fmt::Display;

use convert_case::{Case, Casing};

use wit_parser::{Field as WitField, Record as WitRecord};

use crate::types::{ConcreteName, Type, TypeMap, TypeName};

use super::Render;

/// Represents the name of a record field in Scala
struct FieldName(String);

impl Display for FieldName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for FieldName {
    fn from(name: String) -> Self {
        Self(name.to_case(Case::Camel))
    }
}

/// Represents a record field in Scala
struct Field {
    /// The field name
    name: FieldName,

    /// The Scala type associated to the field
    ty: Type,
}

impl Field {
    // Constructs a Field from WIT
    pub fn from_wit(field: WitField, type_map: &TypeMap) -> Self {
        Self {
            name: FieldName::from(field.name),
            ty: Type::from_wit(field.ty, type_map),
        }
    }
}

/// Represents a record in Scala
pub struct Record {
    /// The record name
    name: TypeName,

    /// The record fields
    fields: Vec<Field>,
}

impl Record {
    // Constructs a Record from WIT
    pub fn from_wit(name: &str, record: &WitRecord, type_map: &TypeMap) -> Self {
        Self {
            name: TypeName::Concrete(ConcreteName::from(name.to_owned())),
            fields: record
                .clone()
                .fields
                .into_iter()
                .map(|field| Field::from_wit(field, type_map))
                .collect(),
        }
    }
}

impl Render for Record {
    fn render(self) -> String {
        let fields = self
            .fields
            .iter()
            .map(|Field { name, ty }| format!("val {name}: {ty}"))
            .collect::<Vec<_>>()
            .join("\n");

        let apply_params = self
            .fields
            .iter()
            .map(|Field { name, ty }| format!("{name}: {ty}"))
            .collect::<Vec<_>>()
            .join(", ");

        let apply_temp_vars = self
            .fields
            .iter()
            .map(|Field { name, ty }| format!("val {name}0: {ty} = {name}"))
            .collect::<Vec<_>>()
            .join("\n");

        let new_vars = self
            .fields
            .iter()
            .map(|Field { name, ty }| format!("val {name}: {ty} = {name}0"))
            .collect::<Vec<_>>()
            .join("\n");

        let name = self.name;

        format!(
            "
                sealed trait {name} extends js.Object {{
                    {fields}
                }}
                object {name} {{
                    def apply({apply_params}): {name} = {{
                        {apply_temp_vars}

                        new {name} {{
                            {new_vars}
                        }}
                    }}
                }}
            "
        )
    }
}
