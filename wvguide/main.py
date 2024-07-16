import argparse
from dataclasses import dataclass
from datetime import datetime
import numpy as np
from scipy.optimize import minimize, OptimizeResult
from typing import Optional, Callable, Union
import logging
import pathlib
import sys

SEED = 0xDEADF00D

np.random.seed(SEED)


logger = logging.getLogger()
logger.setLevel(logging.INFO)
logger.addHandler(logging.StreamHandler(sys.stdout))


_TEST_SHAPE_DETECTOR = (100, 100)
_TEST_NUM_VARIABLES = 10


_NOISE = np.random.normal(scale=0.2, size=_TEST_SHAPE_DETECTOR)


def test_raytrace(x: np.ndarray):
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
    ap = argparse.ArgumentParser()
    ap.add_argument('-o', '--output', default='.')
    ap.add_argument('-i', '--input', default=None, help='directory with optimizer state from which start optimization')
    ap.add_argument('-s', '--steps', type=int, default=1_000_000, help='max number of optimizer steps')
    ap.add_argument('--no-display', action='store_true', help='disable verbose while optimization')
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
    return (np.random.rand(shape) - 0.5) * 50


def optimizer_step(
        fun_loss: Callable,
        x: np.ndarray,
        bounds,
        out,
        n_iter: int = 0,
        jac=None,
        hess=None,
        hessp=None,
        method: str = _METHOD_DEFAULT,
        tol: float = 1e-9,
        callback: Optional[Callable] = None,
        disp: bool = True,
):
    res = minimize(
        fun=fun_loss,
        x0=x,
        jac=jac,
        hess=hess,
        hessp=hessp,
        method=method,
        bounds=bounds,
        tol=tol,
        callback=callback,
        options={'disp': disp, 'maxiter': 1}
    )
    save_state(res, out, n_iter)
    return res


def main(args: argparse.Namespace):
    dt = datetime.now()
    out = pathlib.Path(args.output) / (f"{dt.year:02d}{dt.month:02d}{dt.day:02d}_"
                                       f"{dt.hour:02d}_{dt.minute:02d}_{dt.second:02d}")
    out.mkdir(exist_ok=True, parents=True)

    x = build_init_arguments()
    bounds = [(-1000, 1000) for _ in range(_TEST_NUM_VARIABLES)]
    jac = None
    hess = None
    hessp = None
    method = _METHOD_DEFAULT
    tol = 1e-9
    callback = get_callback_save_state_func(output_dir=out)
    n_iter = 0
    for n_step in range(args.steps):
        print(x)
        res = optimizer_step(
            fun_loss=loss_function,
            x=x,
            out=out,
            bounds=bounds,
            jac=jac,
            hess=hess,
            hessp=hessp,
            method=method,
            tol=tol,
            callback=None,
            disp=True,
            n_iter=n_iter
        )
        n_iter += 1
        print(res.x)
        print('res.hess_inv.todense() ', res.hess_inv.todense())
        print('res.jac', res.jac)
        print(dir(res))


def save_state(res: OptimizeResult, out, n_iter: int):
    out = pathlib.Path(out) / f"{n_iter:04d}"
    out.mkdir(exist_ok=True, parents=True)
    to_save = {}
    for field in ('x', 'hess_inv', 'jac', 'nfev', 'nit', 'njev', 'status'):
        field_value = getattr(res, field, None)
        if field_value is None:
            continue
        elif field == 'hess_inv':
            field_value = res.hess_inv.todense()

        to_save[field] = field_value
    np.savez(out / 'res.array', **to_save)


def get_callback_save_state_func(output_dir: Union[str, pathlib.Path]):
    counter = 0

    def callback_save_state(intermediate_result: OptimizeResult):
        nonlocal counter
        print(dir(intermediate_result))
        out = pathlib.Path(output_dir) / f"{counter:03d}"
        out.mkdir(exist_ok=True, parents=True)
        np.savez(out / 'res.array', x=intermediate_result.x, fun=intermediate_result.fun)
        print('intermediate_result ', intermediate_result)
        counter += 1
    return callback_save_state


if __name__ == '__main__':
    main(get_args())
