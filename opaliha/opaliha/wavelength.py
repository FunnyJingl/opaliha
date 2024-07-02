import numpy as np


class Wavelength:
    def __init__(self, value_nm: float = 555.):
        self._value_nm = value_nm

    @property
    def value(self):
        return self._value_nm

    @property
    def value_nm(self):
        return self._value_nm

    @property
    def value_um(self):
        return self._value_nm / 1000
