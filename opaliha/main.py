import argparse
from opaliha import optical_elements
from opaliha import field_data
from opaliha import wavelength

import matplotlib.pyplot as plt


def get_args():
    ap = argparse.ArgumentParser()
    # ap.add_argument()
    return ap.parse_args()


def main(args: argparse.Namespace) -> None:
    optsys = optical_elements.parse_config('./configs/apochromat3.yaml')
    print(optsys)
    optsys.field = field_data.FieldData(
        field_type=field_data.FieldType.ANGLE,  # 'obj_height', parax im height, real im height
        field_table=[
            field_data.FieldRow(x=0., y=0., weight=1.0, vdx=0., vdy=0., vcx=0., vcy=0., van=0.),
            field_data.FieldRow(x=1., y=0., weight=1.0, vdx=0., vdy=0., vcx=0., vcy=0., van=0.),
            field_data.FieldRow(x=3., y=0., weight=1.0, vdx=0., vdy=0., vcx=0., vcy=0., van=0.),
            field_data.FieldRow(x=5., y=0., weight=1.0, vdx=0., vdy=0., vcx=0., vcy=0., van=0.),
        ]
    )
    optsys.wavelength = [
        wavelength.Wavelength(value_nm=550),
        wavelength.Wavelength(value_nm=380),
        wavelength.Wavelength(value_nm=720),
    ]

    optsys.plot_system()
    # plt.show()
    optsys.plot_trace()
    # g = materials.GlassCatalog('schott').from_agf('/home/funnyjingl/ambar/dev/opaliha/Zemax/Glasscat/SCHOTT.AGF')


if __name__ == '__main__':
    main(get_args())
