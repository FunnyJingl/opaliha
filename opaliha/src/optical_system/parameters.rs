enum FieldType {
    AngleDeg,
    ObjectHeight,
    ParaxImageHeight,
    RealImageHeight,
}

enum FieldNormalization {
    RADIAL,
}

pub struct FieldRaw {
    field_type: FieldType,
    xfield: f64,
    yfield: f64,
    weight: f64,
    vdy: f64,
    vcx: f64,
    vcy: f64,
    van: f64,
}

pub struct FieldData {
    rows: Vec<FieldRaw>,
}

pub struct SequentialParameters {
    pub field_data: FieldData,
    pub name: String,
}
