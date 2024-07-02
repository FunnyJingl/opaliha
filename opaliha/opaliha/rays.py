from __future__ import annotations
import numpy as np
from opaliha import field_data, omath, optical_elements
from opaliha.wavelength import Wavelength
from opaliha import materials
from typing import Optional, Union
from pydantic import BaseModel


class Point(BaseModel):
    x: float = 0.
    y: float = 0.
    z: float = 0.

    @property
    def p(self) -> np.ndarray:
        return np.array([self.x, self.y, self.z])

    def __sub__(self, point: Point) -> Ray:
        return Ray(kx=self.x - point.x,
                   ky=self.y - point.y,
                   kz=self.z - point.z)

    def __add__(self, point: Point) -> Point:
        if isinstance(point, Point):
            return Point(x=self.x + point.x,
                         y=self.y + point.y,
                         z=self.z + point.z)
        elif isinstance(point, np.ndarray):
            return Point(x=self.x + point[0],
                         y=self.y + point[1],
                         z=self.z + point[2])

    def is_in_sphere(self, point: Point, radius: float) -> bool:
        return np.sqrt(
            (self.x - point.x) ** 2 +
            (self.y - point.y) ** 2 +
            (self.z - point.z) ** 2
        ) <= radius

    def __str__(self):
        return f"x={self.x:5f}, y={self.y:5f}, z={self.z:5f}"


class Ray:
    def __init__(
            self,
            kx: float = 0.,
            ky: float = 0.,
            kz: float = 1.,
            origin: Point = Point(x=0, y=0, z=0),
            wavelength: Wavelength = Wavelength()
    ):
        self._kx: float = kx
        self._ky: float = ky
        self._kz: float = kz
        self._origin: Point = origin
        self._history = []
        self._wavelength = wavelength
        self._normalize_vector()

    @property
    def norm(self):
        return np.linalg.norm(self.k)

    def _normalize_vector(self):
        if not np.isclose(self.norm, 1.):
            self._kx = self._kx / self.norm
            self._ky = self._ky / self.norm
            self._kz = self._kz / self.norm

    @property
    def k(self):
        return np.array([self.kx, self.ky, self.kz])

    @property
    def kx(self):
        return self._kx

    @kx.setter
    def kx(self, new_value: float):
        self.kx = new_value

    @property
    def ky(self):
        return self._ky

    @ky.setter
    def ky(self, new_value: float):
        self.ky = new_value

    @property
    def kz(self):
        return self._kz

    @kz.setter
    def kz(self, new_value: float):
        self.kz = new_value

    @property
    def origin(self):
        return self._origin

    @property
    def wavelength(self) -> Wavelength:
        return self._wavelength

    @wavelength.setter
    def wavelength(self, other_wavelength: Wavelength):
        self._wavelength = other_wavelength

    def from_end_point_and_plane(
            self,
            point: Point,
            plane: omath.Planes,
            field: field_data.FieldRow
    ):
        if plane == omath.Planes.YOZ:
            # todo test for all cases - now support only for some y value
            self._ky = np.sin(field.y_rad)
            self._kz = np.cos(field.y_rad)
            self._origin = Point(x=point.x, y=point.y-self._ky, z=point.z-self._kz)
            return self
        else:
            raise Exception()

    def trace(self, optical_elements_list: list[optical_elements.Surface]):
        for n_oe, oe in enumerate(optical_elements_list):
            prev_material = materials.Glass('air') if n_oe == 0 else optical_elements_list[n_oe-1].material
            new_ray = oe.trace(self, prev_material=prev_material)
            self._history.append(self._kx)
            self.update(new_ray)

    def update(self, another_ray: Ray):
        self._kx = another_ray.kx
        self._ky = another_ray.ky
        self._kz = another_ray.kz
        self._origin = another_ray.origin

    def propagate_into_z(self, z: float) -> Point:
        assert self.origin.z <= z
        return Point(
            x=(z - self.origin.z) / np.arccos(self.kx),
            y=(z - self.origin.z) / np.arccos(self.ky),
            z=z,
        )

    def intersect_with_sphere(self, sphere_center: Point, radius: float) -> Optional[Point]:
        a = np.dot(self.k, self.k)
        oc = self.origin - sphere_center
        b = 2.0 * np.dot(oc.k, self.k)
        c = np.dot(oc.k, oc.k) - radius * radius
        discriminant = b * b - 4 * a * c

        if discriminant < 0:
            return None
        elif discriminant == 0:
            t = -b / (2 * a)
            intersection_point = self.origin + t * self.k
            return intersection_point
        else:
            # Two intersections
            sqrt_discriminant = np.sqrt(discriminant)
            t1 = (-b - sqrt_discriminant) / (2 * a)
            t2 = (-b + sqrt_discriminant) / (2 * a)
            intersection_point1 = self.origin + t1 * self.k
            intersection_point2 = self.origin + t2 * self.k
            # check which point is met first and relevant
            if self.origin.is_in_sphere(sphere_center, radius):
                return intersection_point2
            return intersection_point1

    def __str__(self):
        return f"Ray origin - {self.origin}, k=[{self.kx:5f}, {self.ky:5f}, {self.kz:5f}]"


def init_ray_y(field: field_data.FieldRow, n_rays: int, surface_coord: float, y_coor: float) -> Ray:
    assert n_rays > 0

    return Ray().from_end_point_and_plane(
        point=Point(x=0., y=y_coor, z=surface_coord),
        plane=omath.Planes.YOZ,
        field=field
    )


def init_rays_y(
        field: field_data.FieldData,
        surface_coord: float,
        y_coords: list[float],
        n_rays: int = 5,
) -> list[Ray]:
    res = []
    for i in field:
        for yi in y_coords:
            res.append(init_ray_y(field=i, n_rays=n_rays, surface_coord=surface_coord, y_coor=yi))
    return res


def trace_ray_array(ray_array: list[Ray], optical_elements_list: list[optical_elements.Surface]) -> list[Ray]:
    for ray in ray_array:
        ray.trace(optical_elements_list)
    return ray_array
