import argparse
from opaliha import optical_elements
from opaliha import omath
from opaliha import configs
from opaliha import materials


def get_args():
    ap = argparse.ArgumentParser()
    # ap.add_argument()
    return ap.parse_args()


def main(args: argparse.Namespace) -> None:
    optsys = optical_elements.parse_config('./configs/apochromat3.yaml')
    print(optsys)

    g = materials.GlassCatalog('schott').from_agf('/home/funnyjingl/ambar/dev/opaliha/Zemax/Glasscat/SCHOTT.AGF')


if __name__ == '__main__':
    main(get_args())
