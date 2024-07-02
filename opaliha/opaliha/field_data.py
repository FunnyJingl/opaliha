from enum import StrEnum
from typing import Optional
import numpy as np


class FieldType(StrEnum):
    ANGLE = 'angle'
    OBJ_HEIGHT = 'obj_height'
    PAR_IMG_HEIGHT = 'parax_img_height'
    REAL_IMG_HEIGHT = 'real_img_height'


class FieldRow:
    def __init__(
            self,
            x: float = 0.,
            y: float = 0.,
            weight: float = 1.,
            vdx: float = 0.,
            vdy: float = 0.,
            vcx: float = 0.,
            vcy: float = 0.,
            van: float = 0.,
    ):
        self._x = x
        self._y = y
        self._weight = weight
        self._vdx = vdx
        self._vdy = vdy
        self._vcx = vcx
        self._vcy = vcy
        self._van = van

    @property
    def y_deg(self):
        return self._y

    @property
    def y_rad(self):
        return np.deg2rad(self._y)

    @property
    def x_deg(self):
        return self._x

    @property
    def x_rad(self):
        return np.deg2rad(self._x)


class FieldData:
    def __init__(self, field_type: FieldType, field_table: Optional[list[FieldRow]] = None):
        self._field_type: FieldType = field_type
        self._field_table = field_table if field_table is not None else []

    def __next__(self):
        for i in self._field_table:
            yield i

    def __iter__(self):
        for i in self._field_table:
            yield i
