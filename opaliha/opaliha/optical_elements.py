from enum import Enum, StrEnum
from opaliha import materials
import yaml
from pydantic import BaseModel
from typing import Optional, Union


def read_yaml(path):
    with open(path, 'r') as f:
        return yaml.safe_load(f)


class OpticalSystemType(StrEnum):
    SEQUENTIAL = "sequential"
    NONSEQUENTIAL = "nonsequential"


class SequentialSurfaceType(StrEnum):
    # BICONIC = 0
    # BICONIC_ZERNIKE = 1
    # CHEBYSHEV_POLYNOMIAL = 2
    # EVEN_ASPHERE = 3
    # EXTENDED_ASPHERE = 4
    # EXTENDED_ODD_ASPHERE = 5
    # EXTENDED_POLYNOMIAL = 6
    # GRID_SAG = 7
    # IRREGULAR = 8
    # ODD_ASPHERE = 9
    # ODD_COSINE = 10
    # OFF_AXIS_CONIC_FREEFORM = 11
    # PERIODIC = 12
    # POLYNOMIAL = 13
    # Q_TYPE_ASPHERE = 14
    # Q_TYPE_FREEFORM = 15
    STANDARD = 'standard'
    # SUPERSONIC = 17
    # TILTED = 18
    # TOROIDAL = 19
    # ZERNIKE_FRINGE_SAG = 20
    # ZERNIKE_STANDARD_SAG = 21
    # ZERNIKE_ANNULAR_STANDARD_SAG = 22


class FloatValue(BaseModel):
    value: float = 0.0
    is_fixed: bool = False
    is_variable: bool = False


class Surface(BaseModel):
    surface_type: SequentialSurfaceType = SequentialSurfaceType.STANDARD
    comment: str = ""
    radius: FloatValue = FloatValue()
    thickness: FloatValue = FloatValue()
    material: materials.Material = materials.Material()
    clear_semi_diameter: FloatValue = FloatValue()
    is_surface_stop: bool = False
    is_surface_global_coordinates_reference: bool = False

    class Config:
        use_enum_values = True


class ConfigParsed(BaseModel):
    elements: list[Surface]
    optical_system_type: str


Element = Union[Surface]


class OpticalSystem:
    def __init__(self, optical_system_type: OpticalSystemType):
        self._optical_system_type = optical_system_type

    def dump_config(self) -> None:
        raise NotImplemented()


class SequentialOpticalSystem(OpticalSystem):
    def __init__(self, elements: Optional[list[Surface]] = None):
        super().__init__(optical_system_type=OpticalSystemType.SEQUENTIAL)
        self._elements: list[Element] = [] if elements is None else elements

    def load_from_elements(self, elements: list[Surface]) -> None:
        self._elements = elements

    def __repr__(self):
        return ""

    def __str__(self):
        return ""


class NonSequentialOpticalSystem(OpticalSystem):
    def __init__(self):
        super().__init__(optical_system_type=OpticalSystemType.NONSEQUENTIAL)


def parse_config(path: str) -> OpticalSystem:
    config_data = read_yaml(path)
    config_parsed = ConfigParsed(
        elements=config_data['optical_system']['elements'],
        optical_system_type=config_data['optical_system']['type']
    )
    opt_sys = SequentialOpticalSystem()
    opt_sys.load_from_elements(elements=config_parsed.elements)
    return opt_sys
