import argparse
from dataclasses import dataclass
import numpy as np
from scipy.optimize import minimize, OptimizeResult
from typing import Callable
import logging

_TEST_SHAPE_DETECTOR = (100, 100)
_TEST_NUM_VARIABLES = 10


_NOISE = np.random.normal(scale=0.2, size=_TEST_SHAPE_DETECTOR)


def test_raytrace(x: np.ndarray):
    # global min for every value is 2
    noise_amplitude = np.mean((x - 10)**2 / 10000.)
    print(noise_amplitude)
    noise = noise_amplitude * _NOISE
    print(np.mean(noise))
    values_detector = 0.9 + noise
    values_detector = np.clip(values_detector, a_min=0., a_max=1.0)  # clip noise on detector values
    print(values_detector[:10, 0])
    return values_detector


@dataclass
class RayTraceResult:
    values_detector: np.ndarray


_METHOD_DEFAULT = 'L-BFGS-B'


def get_args() -> argparse.Namespace:
    ap = argparse.ArgumentParser('-o', '--output')
    return ap.parse_args()


def loss_efficiency(values_detector) -> float:
    # assumption - overall sum of energy of detector is normalized on energy from light source
    return np.sum(values_detector) / values_detector.size


def loss_uniformity(values_detector: np.ndarray) -> float:
    return np.std(values_detector)


def trace_rays(x) -> RayTraceResult:
    return RayTraceResult(test_raytrace(x))


def l1_loss(y_pred, y_true, w):
    return w * np.abs(y_pred - y_true)


def l2_loss(y_pred, y_true, w):
    return w * (y_pred - y_true) ** 2


def get_loss_f(loss_f_type: str) -> Callable:
    if loss_f_type == 'l1':
        return l1_loss
    elif loss_f_type == 'l2':
        return l2_loss
    else:
        raise NotImplementedError()


def loss_function(
        x,
        w_efficiency: float = 1.,
        efficiency_loss='l2',
        w_uniformity: float = 1.,
        uniformity_loss='l2',
) -> float:
    # return loss function
    trace_rays_result = trace_rays(x)
    values_detector = trace_rays_result.values_detector

    l_eff = get_loss_f(efficiency_loss)(loss_efficiency(values_detector), 1., w_efficiency)
    l_uni = get_loss_f(uniformity_loss)(loss_uniformity(values_detector), 0., w_uniformity)
    loss = l_eff + l_uni
    print('loss - ', loss, 'l eff - ', l_eff, 'l uni - ', l_uni)
    return loss


def build_init_arguments(shape: int = _TEST_NUM_VARIABLES) -> np.ndarray:
    return (np.random.rand(shape) - 0.5) * 1000


def main(args: argparse.Namespace):

    x = build_init_arguments()
    print(x)
    res = minimize(
        fun=loss_function,
        x0=x,
        jac=None,
        hess=None,
        hessp=None,
        method=_METHOD_DEFAULT,
        bounds=[(-1000, 1000) for _ in range(_TEST_NUM_VARIABLES)],
        tol=1e-9,
        callback=callback_save_state,
        options={'disp': True}
    )
    print(res.x)


def callback_save_state(intermediate_result: OptimizeResult):
    print(intermediate_result)


def optimize():
    pass


if __name__ == '__main__':
    main(get_args())