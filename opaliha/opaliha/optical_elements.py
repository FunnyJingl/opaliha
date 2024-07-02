from enum import StrEnum

import numpy as np

from opaliha import materials
import yaml
from pydantic import BaseModel
from typing import Optional, Union, Any
from opaliha import field_data, wavelength, rays, aperture_params
from opaliha.rays import Point
import matplotlib.pyplot as plt
from tabulate import tabulate


_OPTSYS_NAME = "OptSys"


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
    material: materials.Material = materials.Glass("air")
    clear_semi_diameter: FloatValue = FloatValue()
    global_coordinates: Point = Point(x=0., y=0., z=0.)
    is_surface_stop: bool = False
    is_surface_global_coordinates_reference: bool = False

    class Config:
        use_enum_values = True

    def model_post_init(self, __context: Any) -> None:
        if self.material.material_type == materials.MaterialType.GLASS:
            self.material = materials.Glass(name=self.material.name)

    def get_sphere_center(self) -> rays.Point:
        if self.is_surface_plane():
            return rays.Point()
        return rays.Point(x=self.global_coordinates.x,
                          y=self.global_coordinates.y,
                          z=self.global_coordinates.z + self.radius.value)

    def plot(self, ax: plt.Axes):
        if np.isclose(self.radius.value, 0.):
            ax.vlines(self.global_coordinates.x, -self.clear_semi_diameter.value, self.clear_semi_diameter.value)
            return
        deflection_arrow = np.abs(self.radius.value) - np.sqrt(self.radius.value**2 - self.clear_semi_diameter.value**2)
        zi = np.linspace(0, deflection_arrow, 3000)
        yi = np.sqrt(self.radius.value**2 - (np.abs(self.radius.value) - zi)**2)
        zi = self.global_coordinates.z + zi if self.radius.value > 0 else self.global_coordinates.z - zi
        ax.plot(zi, yi, color='k')
        ax.plot(zi, -yi, color='k')

    def get_data_for_table(self) -> dict:
        res = {}
        for field in self.__fields__:
            if isinstance(getattr(self, field), FloatValue):
                res[field] = getattr(self, field).value
                continue
            res[field] = getattr(self, field)
        return res

    def is_surface_plane(self) -> bool:
        return np.isclose(self.radius.value, 0.)

    def trace(self, ray: rays.Ray, prev_material: materials.Material) -> rays.Ray:
        if self.is_surface_plane():
            return self.trace_on_plane(ray, prev_material=prev_material)
        return self.trace_on_sphere(ray, prev_material=prev_material)

    def trace_on_sphere(self, ray: rays.Ray, prev_material: materials.Material) -> rays.Ray:
        wv = ray.wavelength
        print(self.material)
        refraction_relation = prev_material.refractive_index(wv) / self.material.refractive_index(wv)
        intersection_point = ray.intersect_with_sphere(
            sphere_center=self.global_coordinates + rays.Point(x=0., y=0., z=self.radius.value),
            radius=self.radius.value)
        norm_vector = intersection_point - self.global_coordinates
        dp = np.dot(ray.k, norm_vector.k)  # dot product of incident ray and norm vector on surface
        res = (refraction_relation * (ray.k - dp * norm_vector.k) +
               np.sign(dp) * np.sqrt(1 - refraction_relation ** 2 * (1 - dp**2)) * norm_vector.k)
        print(res)
        return rays.Ray(kx=res[0], ky=res[1], kz=res[2], origin=intersection_point)

    def trace_on_plane(self, ray: rays.Ray, prev_material: materials.Material) -> rays.Ray:
        print(ray)
        tir_critical_angle = np.arcsin(
            self.material.refractive_index(ray.wavelength) / prev_material.refractive_index(ray.wavelength)
        )

        # find intersection wiht plane
        new_z = ray.propagate_into_z(self.global_coordinates.z)

        new_angle = prev_material.refractive_index(ray.wavelength) / self.material.refractive_index(ray.wavelength)
        return rays.Ray()


class ConfigParsed(BaseModel):
    elements: list[Surface]
    optical_system_type: str


Element = Union[Surface]


class OpticalSystem:
    def __init__(
            self,
            optical_system_type: OpticalSystemType,
            field: Optional[field_data.FieldData] = None,
            wavelength_table: Optional[wavelength.Wavelength] = None,
            name="OptSys"
    ):
        self._optical_system_type = optical_system_type
        self._field: Optional[field_data.FieldData] = field
        self._wavelength: wavelength.Wavelength = wavelength_table
        self._name = name

    def dump_config(self) -> None:
        raise NotImplemented()

    @property
    def field(self):
        return self._field

    @field.setter
    def field(self, new_field: field_data.FieldData):
        self._field = new_field

    @property
    def wavelength_table(self):
        return self._wavelength

    @wavelength_table.setter
    def wavelength_table(self, new_wavelenght: wavelength.Wavelength):
        self._wavelength = new_wavelenght

    def plot_trace(self):
        raise NotImplemented()


class SequentialOpticalSystem(OpticalSystem):
    def __init__(
            self,
            aperture: aperture_params.Aperture = aperture_params.Aperture(
                aperture_type=aperture_params.ApertureType.ENTRANCE_PUPIL_DIAMETER,
                aperture_value=10.
            ),
            elements: Optional[list[Surface]] = None,
            field: Optional[field_data.FieldData] = None,
            name: str = _OPTSYS_NAME
    ):
        super().__init__(optical_system_type=OpticalSystemType.SEQUENTIAL, name=name)
        self._elements: list[Element] = [] if elements is None else elements
        self._field: field_data.FieldData = field if field is not None else field_data.FieldData(
            field_type=field_data.FieldType.ANGLE
        )
        self._aperture: aperture_params.Aperture = aperture

    def load_from_elements(self, elements: list[Surface]) -> None:
        self._elements = elements
        self._update_coordinates()

    def plot_trace(self, n_rays: int = 5):
        traced_rays = self._trace_rays(n_rays=n_rays)

        fig, ax = self._vis_figure()
        self._vis_optsys(ax=ax)
        # self._vis_traced_rays(traced_rays=traced_rays, ax=ax)

    def plot_system(self):
        fig, ax = self._vis_figure()
        self._vis_optsys(ax=ax)

    def _vis_figure(self):
        fig, ax = plt.subplots(figsize=(16, 9))
        return fig, ax

    def _vis_optsys(self, ax: plt.Axes):
        for i in self._elements:
            i.plot(ax)
        return ax

    def _vis_traced_rays(self):
        pass

    def _trace_rays(self, n_rays: int) -> list[rays.Ray]:
        if n_rays == 1:
            y_coords = [0.,]
        else:
            y_coords = list(np.linspace(-self._aperture.value / 2, +self._aperture.value / 2, n_rays))

        ray_array = rays.init_rays_y(
            field=self._field,
            surface_coord=self.start_pos,
            y_coords=y_coords
        )
        ray_array = rays.trace_ray_array(
            ray_array=ray_array,
            optical_elements_list=self._elements
        )
        return ray_array

    @property
    def start_pos(self):
        # todo remove hardcode
        return 0.

    def __repr__(self):
        return ""

    def __str__(self):
        res = f"Optycal system {self._name}\n"
        data = []
        for ni, i in enumerate(self._elements):
            data.append(i.get_data_for_table())
        res += tabulate(data, headers='keys')
        return res

    def _update_coordinates(self):
        n_elements = len(self._elements)
        if n_elements > 0:
            self._elements[0].global_coordinates.z = self.start_pos
            if n_elements > 1:
                for i in range(1, n_elements):
                    self._elements[i].global_coordinates.z += (self._elements[i-1].global_coordinates.z +
                                                               self._elements[i-1].thickness.value)


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
