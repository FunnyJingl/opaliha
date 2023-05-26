enum FieldType {
    ANGLE_DEG,
    OJECT_HEIGHT,
    PARAX_IMAGE_HEIGHT,
    REAL_IMAGE_HEIGHT,
}

enum FieldNormalization {
    RADIAL,
}

pub struct FieldRaw {
    field_type: FieldType,
    xfield: float,
    yfield: float,
    weight: float,
    vdx: float,
    vdy: float,
    vcx: float,
    vcy: float,
    van: float,
}

pub struct FieldData {
    rows: Vec<FieldRaw>,
}
