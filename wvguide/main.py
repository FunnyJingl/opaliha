import argparse
from dataclasses import dataclass
import numpy as np
from scipy.optimize import minimize, OptimizeResult
from typing import Callable
import logging

_TEST_SHAPE_DETECTOR = (100, 100)
_TEST_NUM_VARIABLES = 10


def test_raytrace(x: np.ndarray):
    # global min for every value is 2
    noise_amplitude = (x - 2)**2 / 100.
    noise = np.abs(np.mean(noise_amplitude)) * np.random.normal(scale=0.1, size=_TEST_SHAPE_DETECTOR)
    values_detector = 0.7 + noise
    values_detector = np.clip(values_detector, a_min=0., a_max=1.0)  # clip noise on detector values
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
        efficiency_loss='l1',
        w_uniformity: float = 1.,
        uniformity_loss='l1',
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
    return (np.random.rand(shape) - 0.5) * 100


def main(args: argparse.Namespace):

    x = build_init_arguments()

    res = minimize(
        fun=loss_function,
        x0=x,
        jac=None,
        hess=None,
        hessp=None,
        method=_METHOD_DEFAULT,
        bounds=[(-50, 50) for _ in range(_TEST_NUM_VARIABLES)],
        tol=None,
        callback=callback_save_state,
        options={'disp': True}
    )
    print(res.x)


def callback_save_state(intermediate_result: OptimizeResult):
    pass


def optimize():
    pass


if __name__ == '__main__':
    main(get_args())
