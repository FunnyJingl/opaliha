from enum import StrEnum


class ApertureType(StrEnum):
    ENTRANCE_PUPIL_DIAMETER = 'entrance_pupil_diameter'


class Aperture:
    def __init__(self, aperture_type: ApertureType, aperture_value: float):
        self._aperture_type = aperture_type
        self._aperture_value = aperture_value

    @property
    def value(self):
        return self._aperture_value
